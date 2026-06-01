; ModuleID = 'builtin.module'
source_filename = "path_tracer"
target datalayout = "e-i64:64-i128:128-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

declare i8 @llvm.fptoui.sat.i8.f64(double)

define ptx_kernel void @render(ptr %v0, i64 %v1, { double, double, double } %v2, { double, double, double } %v3, { double, double, double } %v4, { double, double, double } %v5, { double, double, double } %v6, { double, double, double } %v7, { double, double, double } %v8, double %v9, i8 %v10, i16 %v11, i16 %v12, ptr %v13, i64 %v14) {
entry:
  %v15 = insertvalue { ptr, i64 } undef, ptr %v0, 0
  %v16 = insertvalue { ptr, i64 } %v15, i64 %v1, 1
  %v17 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } undef, { double, double, double } %v2, 0
  %v18 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v17, { double, double, double } %v3, 1
  %v19 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v18, { double, double, double } %v4, 2
  %v20 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v19, { double, double, double } %v5, 3
  %v21 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v20, { double, double, double } %v6, 4
  %v22 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v21, { double, double, double } %v7, 5
  %v23 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v22, { double, double, double } %v8, 6
  %v24 = insertvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v23, double %v9, 7
  %v25 = insertvalue { ptr, i64 } undef, ptr %v13, 0
  %v26 = insertvalue { ptr, i64 } %v25, i64 %v14, 1
  br label %bb0
bb0:
  %v27 = phi { ptr, i64 } [ %v16, %entry ]
  %v28 = phi { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } [ %v24, %entry ]
  %v29 = phi i8 [ %v10, %entry ]
  %v30 = phi i16 [ %v11, %entry ]
  %v31 = phi i16 [ %v12, %entry ]
  %v32 = phi { ptr, i64 } [ %v26, %entry ]
  %v33 = alloca { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }
  %v34 = alloca {  }
  %v35 = alloca i32
  %v36 = alloca { { double, double, double }, { double, double, double } }
  store { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v28, ptr %v33
  %v37 = bitcast ptr %v34 to ptr
  %v38 = call i64 @cuda_device____internal__index_1d(ptr %v37)
  br label %bb1
bb1:
  %v39 = trunc i64 %v38 to i32
  %v40 = add i32 %v39, 23712
  store i32 %v40, ptr %v35
  %v41 = extractvalue { ptr, i64 } %v32, 1
  %v42 = icmp ult i64 %v38, %v41
  %v43 = xor i1 %v42, 1
  br i1 %v43, label %bb14, label %bb13
bb2:
  %v44 = extractvalue { i8, ptr } %v86, 1
  %v45 = zext i16 %v30 to i64
  %v46 = icmp eq i64 %v45, 0
  %v47 = xor i1 %v46, 1
  br i1 %v47, label %bb3, label %bb26
bb3:
  %v48 = udiv i64 %v38, %v45
  %v49 = zext i16 %v31 to i64
  %v50 = icmp eq i64 %v49, 0
  %v51 = xor i1 %v50, 1
  br i1 %v51, label %bb4, label %bb27
bb4:
  %v52 = udiv i64 %v38, %v49
  br label %bb5
bb5:
  %v53 = phi i8 [ 0, %bb4 ], [ %v96, %bb10 ]
  %v54 = phi double [ 0.0, %bb4 ], [ %v76, %bb10 ]
  %v55 = phi double [ 0.0, %bb4 ], [ %v78, %bb10 ]
  %v56 = phi double [ 0.0, %bb4 ], [ %v80, %bb10 ]
  %v57 = icmp ult i8 %v53, %v29
  %v58 = xor i1 %v57, 1
  br i1 %v58, label %bb18, label %bb17
bb6:
  unreachable
bb7:
  %v59 = uitofp i64 %v48 to double
  %v60 = call double @randf(ptr %v35)
  br label %bb21
bb8:
  %v61 = uitofp i8 %v29 to double
  %v62 = fdiv double %v54, %v61
  %v63 = fdiv double %v55, %v61
  %v64 = fdiv double %v56, %v61
  %v65 = call double @sqrt_f64(double %v62)
  br label %bb23
bb9:
  %v66 = bitcast ptr %v36 to ptr
  %v67 = insertvalue { double, double, double } undef, double 0.0, 0
  %v68 = insertvalue { double, double, double } %v67, double 0.0, 1
  %v69 = insertvalue { double, double, double } %v68, double 0.0, 2
  %v70 = insertvalue { i8, { double, double, double }, { double, double, double }, double, double } undef, i8 0, 0
  %v71 = insertvalue { i8, { double, double, double }, { double, double, double }, double, double } %v70, { double, double, double } %v69, 1
  %v72 = extractvalue { ptr, i64 } %v27, 0
  %v73 = extractvalue { ptr, i64 } %v27, 1
  %v74 = call { double, double, double } @path_tracer__path_tracer__get_color(ptr %v66, ptr %v72, i64 %v73, i8 50, { i8, { double, double, double }, { double, double, double }, double, double } %v71, ptr %v35)
  br label %bb10
bb10:
  %v75 = extractvalue { double, double, double } %v74, 0
  %v76 = fadd double %v54, %v75
  %v77 = extractvalue { double, double, double } %v74, 1
  %v78 = fadd double %v55, %v77
  %v79 = extractvalue { double, double, double } %v74, 2
  %v80 = fadd double %v56, %v79
  br label %bb5
bb11:
  br label %bb12
bb12:
  ret void
bb13:
  %v81 = extractvalue { ptr, i64 } %v32, 0
  %v82 = getelementptr inbounds { i8, i8, i8 }, ptr %v81, i64 %v38
  %v83 = insertvalue { i8, ptr } undef, i8 1, 0
  %v84 = insertvalue { i8, ptr } %v83, ptr %v82, 1
  br label %bb15
bb14:
  %v85 = insertvalue { i8, ptr } undef, i8 0, 0
  br label %bb15
bb15:
  %v86 = phi { i8, ptr } [ %v84, %bb13 ], [ %v85, %bb14 ]
  %v87 = extractvalue { i8, ptr } %v86, 0
  %v88 = zext i8 %v87 to i64
  %v89 = icmp eq i64 %v88, 1
  br i1 %v89, label %bb2, label %bb16
bb16:
  %v90 = icmp eq i64 %v88, 0
  br i1 %v90, label %bb11, label %bb6
bb17:
  %v91 = add i8 %v53, 1
  %v92 = insertvalue { i8, i8 } undef, i8 1, 0
  %v93 = insertvalue { i8, i8 } %v92, i8 %v53, 1
  br label %bb19
bb18:
  %v94 = insertvalue { i8, i8 } undef, i8 0, 0
  br label %bb19
bb19:
  %v95 = phi { i8, i8 } [ %v93, %bb17 ], [ %v94, %bb18 ]
  %v96 = phi i8 [ %v91, %bb17 ], [ %v53, %bb18 ]
  %v97 = extractvalue { i8, i8 } %v95, 0
  %v98 = zext i8 %v97 to i64
  %v99 = icmp eq i64 %v98, 0
  br i1 %v99, label %bb8, label %bb20
bb20:
  %v100 = icmp eq i64 %v98, 1
  br i1 %v100, label %bb7, label %bb6
bb21:
  %v101 = fadd double %v59, %v60
  %v102 = uitofp i16 %v30 to double
  %v103 = fdiv double %v101, %v102
  %v104 = uitofp i64 %v52 to double
  %v105 = call double @randf(ptr %v35)
  br label %bb22
bb22:
  %v106 = fadd double %v104, %v105
  %v107 = uitofp i16 %v31 to double
  %v108 = fdiv double %v106, %v107
  %v109 = bitcast ptr %v33 to ptr
  %v110 = call { { double, double, double }, { double, double, double } } @path_tracer__path_tracer__camera__Camera__get_ray(ptr %v109, double %v103, double %v108, ptr %v35)
  store { { double, double, double }, { double, double, double } } %v110, ptr %v36
  br label %bb9
bb23:
  %v111 = call double @sqrt_f64(double %v63)
  br label %bb24
bb24:
  %v112 = call double @sqrt_f64(double %v64)
  br label %bb25
bb25:
  %v113 = fmul double %v65, 255.99
  %v114 = call i8 @llvm.fptoui.sat.i8.f64(double %v113)
  %v115 = fmul double %v111, 255.99
  %v116 = call i8 @llvm.fptoui.sat.i8.f64(double %v115)
  %v117 = fmul double %v112, 255.99
  %v118 = call i8 @llvm.fptoui.sat.i8.f64(double %v117)
  %v119 = insertvalue { i8, i8, i8 } undef, i8 %v114, 0
  %v120 = insertvalue { i8, i8, i8 } %v119, i8 %v116, 1
  %v121 = insertvalue { i8, i8, i8 } %v120, i8 %v118, 2
  store { i8, i8, i8 } %v121, ptr %v44
  br label %bb12
bb26:
  unreachable
bb27:
  unreachable
}

declare i32 @llvm.nvvm.read.ptx.sreg.tid.x()
declare i32 @llvm.nvvm.read.ptx.sreg.ctaid.x()
declare i32 @llvm.nvvm.read.ptx.sreg.ntid.x()

define i64 @cuda_device____internal__index_1d(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = call i32 @llvm.nvvm.read.ptx.sreg.tid.x()
  br label %bb1
bb1:
  %v3 = zext i32 %v2 to i64
  %v4 = call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x()
  br label %bb2
bb2:
  %v5 = zext i32 %v4 to i64
  %v6 = call i32 @llvm.nvvm.read.ptx.sreg.ntid.x()
  br label %bb3
bb3:
  %v7 = zext i32 %v6 to i64
  %v8 = mul i64 %v5, %v7
  %v9 = add i64 %v8, %v3
  ret i64 %v9
}

define double @randf(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = call i32 @xor_shift(ptr %v1)
  br label %bb1
bb1:
  %v3 = uitofp i32 %v2 to double
  %v4 = fdiv double %v3, 4294967296.0
  ret double %v4
}

define double @sqrt_f64(double %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi double [ %v0, %entry ]
  %v2 = fcmp oeq double %v1, 0.0
  %v3 = xor i1 %v2, 1
  br i1 %v3, label %bb1, label %bb2
bb1:
  %v4 = fcmp oeq double %v1, 1.0
  %v5 = xor i1 %v4, 1
  br i1 %v5, label %bb3, label %bb2
bb2:
  br label %bb8
bb3:
  br label %bb4
bb4:
  %v6 = phi double [ %v1, %bb3 ], [ %v12, %bb6 ]
  %v7 = phi i32 [ 0, %bb3 ], [ %v22, %bb6 ]
  %v8 = icmp slt i32 %v7, 20
  %v9 = xor i1 %v8, 1
  br i1 %v9, label %bb10, label %bb9
bb5:
  unreachable
bb6:
  %v10 = fdiv double %v1, %v6
  %v11 = fadd double %v6, %v10
  %v12 = fmul double 0.5, %v11
  br label %bb4
bb7:
  br label %bb8
bb8:
  %v13 = phi double [ %v1, %bb2 ], [ %v6, %bb7 ]
  ret double %v13
bb9:
  %v14 = add i32 %v7, 1
  %v15 = insertvalue { i32, i1 } undef, i32 %v14, 0
  %v16 = insertvalue { i32, i1 } %v15, i1 0, 1
  %v17 = extractvalue { i32, i1 } %v16, 0
  %v18 = extractvalue { i32, i1 } %v16, 1
  %v19 = xor i1 %v18, 1
  br i1 %v19, label %bb14, label %bb13
bb10:
  %v20 = insertvalue { i8, i32 } undef, i8 0, 0
  br label %bb11
bb11:
  %v21 = phi { i8, i32 } [ %v20, %bb10 ], [ %v28, %bb14 ]
  %v22 = phi i32 [ %v7, %bb10 ], [ %v17, %bb14 ]
  %v23 = extractvalue { i8, i32 } %v21, 0
  %v24 = zext i8 %v23 to i64
  %v25 = icmp eq i64 %v24, 0
  br i1 %v25, label %bb7, label %bb12
bb12:
  %v26 = icmp eq i64 %v24, 1
  br i1 %v26, label %bb6, label %bb5
bb13:
  br label %bb5
bb14:
  %v27 = insertvalue { i8, i32 } undef, i8 1, 0
  %v28 = insertvalue { i8, i32 } %v27, i32 %v7, 1
  br label %bb11
}

define { double, double, double } @path_tracer__path_tracer__get_color(ptr %v0, ptr %v1, i64 %v2, i8 %v3, { i8, { double, double, double }, { double, double, double }, double, double } %v4, ptr %v5) {
entry:
  %v6 = insertvalue { ptr, i64 } undef, ptr %v1, 0
  %v7 = insertvalue { ptr, i64 } %v6, i64 %v2, 1
  br label %bb0
bb0:
  %v8 = phi ptr [ %v0, %entry ]
  %v9 = phi { ptr, i64 } [ %v7, %entry ]
  %v10 = phi i8 [ %v3, %entry ]
  %v11 = phi { i8, { double, double, double }, { double, double, double }, double, double } [ %v4, %entry ]
  %v12 = phi ptr [ %v5, %entry ]
  %v13 = alloca { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }
  %v14 = alloca { { double, double, double }, { double, double, double } }
  %v15 = alloca { double, double, double }
  %v16 = insertvalue { double, double, double } undef, double 0.0, 0
  %v17 = insertvalue { double, double, double } %v16, double 0.0, 1
  %v18 = insertvalue { double, double, double } %v17, double 0.0, 2
  %v19 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } undef, { i8, { double, double, double }, { double, double, double }, double, double } %v11, 0
  %v20 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v19, double -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 1
  %v21 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v20, { double, double, double } %v18, 2
  %v22 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v21, { double, double, double } %v18, 3
  store { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v22, ptr %v13
  %v23 = extractvalue { ptr, i64 } %v9, 0
  %v24 = extractvalue { ptr, i64 } %v9, 1
  %v25 = call i1 @path_tracer__path_tracer__check_hits(ptr %v8, double 0.001, double 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, ptr %v13, ptr %v23, i64 %v24, { i8, { double, double, double }, { double, double, double }, double, double } %v11)
  br label %bb1
bb1:
  %v26 = xor i1 %v25, 1
  br i1 %v26, label %bb8, label %bb2
bb2:
  %v27 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v18, 0
  %v28 = insertvalue { { double, double, double }, { double, double, double } } %v27, { double, double, double } %v18, 1
  store { { double, double, double }, { double, double, double } } %v28, ptr %v14
  store { double, double, double } %v18, ptr %v15
  %v29 = icmp ult i8 %v10, 50
  %v30 = xor i1 %v29, 1
  br i1 %v30, label %bb7, label %bb3
bb3:
  %v31 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v13
  %v32 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v31, 0
  %v33 = bitcast ptr %v13 to ptr
  %v34 = call i1 @scatter({ i8, { double, double, double }, { double, double, double }, double, double } %v32, ptr %v8, ptr %v33, ptr %v15, ptr %v14, ptr %v12)
  br label %bb12
bb4:
  %v35 = load { double, double, double }, ptr %v15
  %v36 = extractvalue { double, double, double } %v35, 0
  %v37 = load { double, double, double }, ptr %v15
  %v38 = extractvalue { double, double, double } %v37, 1
  %v39 = load { double, double, double }, ptr %v15
  %v40 = extractvalue { double, double, double } %v39, 2
  %v41 = bitcast ptr %v14 to ptr
  %v42 = add i8 %v10, 1
  %v43 = extractvalue { ptr, i64 } %v9, 0
  %v44 = extractvalue { ptr, i64 } %v9, 1
  %v45 = call { double, double, double } @path_tracer__path_tracer__get_color(ptr %v41, ptr %v43, i64 %v44, i8 %v42, { i8, { double, double, double }, { double, double, double }, double, double } %v11, ptr %v12)
  br label %bb5
bb5:
  %v46 = extractvalue { double, double, double } %v45, 0
  %v47 = fmul double %v36, %v46
  %v48 = extractvalue { double, double, double } %v45, 1
  %v49 = fmul double %v38, %v48
  %v50 = extractvalue { double, double, double } %v45, 2
  %v51 = fmul double %v40, %v50
  %v52 = insertvalue { double, double, double } undef, double %v47, 0
  %v53 = insertvalue { double, double, double } %v52, double %v49, 1
  %v54 = insertvalue { double, double, double } %v53, double %v51, 2
  br label %bb10
bb6:
  br label %bb7
bb7:
  br label %bb10
bb8:
  %v55 = getelementptr inbounds { { double, double, double }, { double, double, double } }, ptr %v8, i32 0, i32 1
  %v56 = call { double, double, double } @path_tracer__path_tracer__vec3__Vec3__to_normalized(ptr %v55)
  br label %bb9
bb9:
  %v57 = extractvalue { double, double, double } %v56, 1
  %v58 = fadd double %v57, 1.0
  %v59 = fmul double 0.5, %v58
  %v60 = fsub double 1.0, %v59
  %v61 = fmul double 1.0, %v60
  %v62 = fmul double 0.5, %v59
  %v63 = fmul double 0.7, %v59
  %v64 = fmul double 1.0, %v59
  %v65 = fadd double %v61, %v62
  %v66 = fadd double %v61, %v63
  %v67 = fadd double %v61, %v64
  %v68 = insertvalue { double, double, double } undef, double %v65, 0
  %v69 = insertvalue { double, double, double } %v68, double %v66, 1
  %v70 = insertvalue { double, double, double } %v69, double %v67, 2
  br label %bb11
bb10:
  %v71 = phi { double, double, double } [ %v54, %bb5 ], [ %v18, %bb7 ]
  br label %bb11
bb11:
  %v72 = phi { double, double, double } [ %v70, %bb9 ], [ %v71, %bb10 ]
  ret { double, double, double } %v72
bb12:
  %v73 = xor i1 %v34, 1
  br i1 %v73, label %bb6, label %bb4
}

define { { double, double, double }, { double, double, double } } @path_tracer__path_tracer__camera__Camera__get_ray(ptr %v0, double %v1, double %v2, ptr %v3) {
entry:
  br label %bb0
bb0:
  %v4 = phi ptr [ %v0, %entry ]
  %v5 = phi double [ %v1, %entry ]
  %v6 = phi double [ %v2, %entry ]
  %v7 = phi ptr [ %v3, %entry ]
  %v8 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v9 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v8, 7
  %v10 = call { double, double, double } @path_tracer__path_tracer__camera__random_in_unit_disk(ptr %v7)
  br label %bb1
bb1:
  %v11 = extractvalue { double, double, double } %v10, 0
  %v12 = fmul double %v11, %v9
  %v13 = extractvalue { double, double, double } %v10, 1
  %v14 = fmul double %v13, %v9
  %v15 = extractvalue { double, double, double } %v10, 2
  %v16 = fmul double %v15, %v9
  %v17 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v18 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v17, 4
  %v19 = extractvalue { double, double, double } %v18, 0
  %v20 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v21 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v20, 4
  %v22 = extractvalue { double, double, double } %v21, 1
  %v23 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v24 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v23, 4
  %v25 = extractvalue { double, double, double } %v24, 2
  %v26 = fmul double %v19, %v12
  %v27 = fmul double %v22, %v12
  %v28 = fmul double %v25, %v12
  %v29 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v30 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v29, 5
  %v31 = extractvalue { double, double, double } %v30, 0
  %v32 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v33 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v32, 5
  %v34 = extractvalue { double, double, double } %v33, 1
  %v35 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v36 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v35, 5
  %v37 = extractvalue { double, double, double } %v36, 2
  %v38 = fmul double %v31, %v14
  %v39 = fmul double %v34, %v14
  %v40 = fmul double %v37, %v14
  %v41 = fadd double %v26, %v38
  %v42 = fadd double %v27, %v39
  %v43 = fadd double %v28, %v40
  %v44 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v45 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v44, 0
  %v46 = extractvalue { double, double, double } %v45, 0
  %v47 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v48 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v47, 0
  %v49 = extractvalue { double, double, double } %v48, 1
  %v50 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v51 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v50, 0
  %v52 = extractvalue { double, double, double } %v51, 2
  %v53 = fadd double %v46, %v41
  %v54 = fadd double %v49, %v42
  %v55 = fadd double %v52, %v43
  %v56 = insertvalue { double, double, double } undef, double %v53, 0
  %v57 = insertvalue { double, double, double } %v56, double %v54, 1
  %v58 = insertvalue { double, double, double } %v57, double %v55, 2
  %v59 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v60 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v59, 1
  %v61 = extractvalue { double, double, double } %v60, 0
  %v62 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v63 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v62, 1
  %v64 = extractvalue { double, double, double } %v63, 1
  %v65 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v66 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v65, 1
  %v67 = extractvalue { double, double, double } %v66, 2
  %v68 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v69 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v68, 2
  %v70 = extractvalue { double, double, double } %v69, 0
  %v71 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v72 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v71, 2
  %v73 = extractvalue { double, double, double } %v72, 1
  %v74 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v75 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v74, 2
  %v76 = extractvalue { double, double, double } %v75, 2
  %v77 = fmul double %v70, %v5
  %v78 = fmul double %v73, %v5
  %v79 = fmul double %v76, %v5
  %v80 = fadd double %v61, %v77
  %v81 = fadd double %v64, %v78
  %v82 = fadd double %v67, %v79
  %v83 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v84 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v83, 3
  %v85 = extractvalue { double, double, double } %v84, 0
  %v86 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v87 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v86, 3
  %v88 = extractvalue { double, double, double } %v87, 1
  %v89 = load { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double }, ptr %v4
  %v90 = extractvalue { { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, double } %v89, 3
  %v91 = extractvalue { double, double, double } %v90, 2
  %v92 = fmul double %v85, %v6
  %v93 = fmul double %v88, %v6
  %v94 = fmul double %v91, %v6
  %v95 = fadd double %v80, %v92
  %v96 = fadd double %v81, %v93
  %v97 = fadd double %v82, %v94
  %v98 = fsub double %v95, %v46
  %v99 = fsub double %v96, %v49
  %v100 = fsub double %v97, %v52
  %v101 = fsub double %v98, %v41
  %v102 = fsub double %v99, %v42
  %v103 = fsub double %v100, %v43
  %v104 = insertvalue { double, double, double } undef, double %v101, 0
  %v105 = insertvalue { double, double, double } %v104, double %v102, 1
  %v106 = insertvalue { double, double, double } %v105, double %v103, 2
  %v107 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v58, 0
  %v108 = insertvalue { { double, double, double }, { double, double, double } } %v107, { double, double, double } %v106, 1
  ret { { double, double, double }, { double, double, double } } %v108
}

define i32 @xor_shift(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = load i32, ptr %v1
  %v3 = and i32 13, 31
  %v4 = shl i32 %v2, %v3
  %v5 = xor i32 %v2, %v4
  %v6 = and i32 17, 31
  %v7 = lshr i32 %v5, %v6
  %v8 = xor i32 %v5, %v7
  %v9 = and i32 5, 31
  %v10 = shl i32 %v8, %v9
  %v11 = xor i32 %v8, %v10
  store i32 %v11, ptr %v1
  ret i32 %v11
}

define i1 @path_tracer__path_tracer__check_hits(ptr %v0, double %v1, double %v2, ptr %v3, ptr %v4, i64 %v5, { i8, { double, double, double }, { double, double, double }, double, double } %v6) {
entry:
  %v7 = insertvalue { ptr, i64 } undef, ptr %v4, 0
  %v8 = insertvalue { ptr, i64 } %v7, i64 %v5, 1
  br label %bb0
bb0:
  %v9 = phi ptr [ %v0, %entry ]
  %v10 = phi double [ %v1, %entry ]
  %v11 = phi double [ %v2, %entry ]
  %v12 = phi ptr [ %v3, %entry ]
  %v13 = phi { ptr, i64 } [ %v8, %entry ]
  %v14 = phi { i8, { double, double, double }, { double, double, double }, double, double } [ %v6, %entry ]
  %v15 = alloca { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }
  %v16 = alloca { { ptr }, ptr }
  %v17 = insertvalue { double, double, double } undef, double 0.0, 0
  %v18 = insertvalue { double, double, double } %v17, double 0.0, 1
  %v19 = insertvalue { double, double, double } %v18, double 0.0, 2
  %v20 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } undef, { i8, { double, double, double }, { double, double, double }, double, double } %v14, 0
  %v21 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v20, double -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 1
  %v22 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v21, { double, double, double } %v19, 2
  %v23 = insertvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v22, { double, double, double } %v19, 3
  store { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v23, ptr %v15
  %v24 = extractvalue { ptr, i64 } %v13, 1
  %v25 = alloca { ptr, i64 }
  store { ptr, i64 } %v13, ptr %v25
  %v26 = load { { ptr, i64 } }, ptr %v25
  %v27 = alloca { { ptr, i64 } }
  store { { ptr, i64 } } %v26, ptr %v27
  %v28 = load { ptr, i64 }, ptr %v27
  %v29 = extractvalue { ptr, i64 } %v28, 0
  %v30 = insertvalue { ptr } undef, ptr %v29, 0
  %v31 = extractvalue { ptr } %v30, 0
  %v32 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v31, i64 %v24
  %v33 = bitcast ptr %v32 to ptr
  %v34 = insertvalue { { ptr }, ptr } undef, { ptr } %v30, 0
  %v35 = insertvalue { { ptr }, ptr } %v34, ptr %v33, 1
  store { { ptr }, ptr } %v35, ptr %v16
  br label %bb1
bb1:
  %v36 = phi i1 [ 0, %bb0 ], [ %v58, %bb11 ]
  %v37 = phi double [ %v11, %bb0 ], [ %v59, %bb11 ]
  %v38 = load { { ptr }, ptr }, ptr %v16
  %v39 = extractvalue { { ptr }, ptr } %v38, 0
  %v40 = load { { ptr }, ptr }, ptr %v16
  %v41 = extractvalue { { ptr }, ptr } %v40, 1
  %v42 = insertvalue { ptr } undef, ptr %v41, 0
  %v43 = extractvalue { ptr } %v39, 0
  %v44 = extractvalue { ptr } %v42, 0
  %v45 = icmp eq ptr %v43, %v44
  %v46 = xor i1 %v45, 1
  br i1 %v46, label %bb15, label %bb14
bb2:
  unreachable
bb3:
  %v47 = extractvalue { i8, ptr } %v60, 1
  %v48 = call i1 @path_tracer__path_tracer__geometry__Triangle__hit(ptr %v47, ptr %v9, double %v10, double %v37, ptr %v15)
  br label %bb5
bb4:
  ret i1 %v36
bb5:
  %v49 = xor i1 %v48, 1
  br i1 %v49, label %bb10, label %bb6
bb6:
  %v50 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  %v51 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v50, 1
  %v52 = fcmp ogt double %v37, %v51
  %v53 = xor i1 %v52, 1
  br i1 %v53, label %bb8, label %bb7
bb7:
  %v54 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  %v55 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v54, 1
  %v56 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  store { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v56, ptr %v12
  br label %bb9
bb8:
  br label %bb9
bb9:
  %v57 = phi double [ %v55, %bb7 ], [ %v37, %bb8 ]
  br label %bb11
bb10:
  br label %bb11
bb11:
  %v58 = phi i1 [ 1, %bb9 ], [ %v36, %bb10 ]
  %v59 = phi double [ %v57, %bb9 ], [ %v37, %bb10 ]
  br label %bb1
bb12:
  %v60 = phi { i8, ptr } [ %v65, %bb14 ], [ %v71, %bb15 ]
  %v61 = extractvalue { i8, ptr } %v60, 0
  %v62 = zext i8 %v61 to i64
  %v63 = icmp eq i64 %v62, 0
  br i1 %v63, label %bb4, label %bb13
bb13:
  %v64 = icmp eq i64 %v62, 1
  br i1 %v64, label %bb3, label %bb2
bb14:
  %v65 = insertvalue { i8, ptr } undef, i8 0, 0
  br label %bb12
bb15:
  %v66 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v43, i64 1
  %v67 = insertvalue { ptr } undef, ptr %v66, 0
  %v68 = getelementptr inbounds { { ptr }, ptr }, ptr %v16, i32 0, i32 0
  store { ptr } %v67, ptr %v68
  %v69 = extractvalue { ptr } %v39, 0
  %v70 = insertvalue { i8, ptr } undef, i8 1, 0
  %v71 = insertvalue { i8, ptr } %v70, ptr %v69, 1
  br label %bb12
}

define i1 @scatter({ i8, { double, double, double }, { double, double, double }, double, double } %v0, ptr %v1, ptr %v2, ptr %v3, ptr %v4, ptr %v5) {
entry:
  br label %bb0
bb0:
  %v6 = phi { i8, { double, double, double }, { double, double, double }, double, double } [ %v0, %entry ]
  %v7 = phi ptr [ %v1, %entry ]
  %v8 = phi ptr [ %v2, %entry ]
  %v9 = phi ptr [ %v3, %entry ]
  %v10 = phi ptr [ %v4, %entry ]
  %v11 = phi ptr [ %v5, %entry ]
  %v12 = extractvalue { i8, { double, double, double }, { double, double, double }, double, double } %v6, 0
  %v13 = zext i8 %v12 to i64
  %v14 = icmp eq i64 %v13, 0
  br i1 %v14, label %bb8, label %bb1
bb1:
  %v15 = icmp eq i64 %v13, 1
  br i1 %v15, label %bb7, label %bb2
bb2:
  %v16 = icmp eq i64 %v13, 2
  br i1 %v16, label %bb6, label %bb3
bb3:
  %v17 = icmp eq i64 %v13, 3
  br i1 %v17, label %bb5, label %bb4
bb4:
  unreachable
bb5:
  %v18 = call i1 @path_tracer__path_tracer__materials__normal_scatter(ptr %v8, ptr %v9, ptr %v10, ptr %v11)
  br label %bb9
bb6:
  %v19 = extractvalue { i8, { double, double, double }, { double, double, double }, double, double } %v6, 4
  %v20 = call i1 @path_tracer__path_tracer__materials__dielectric_scatter(double %v19, ptr %v7, ptr %v8, ptr %v9, ptr %v10, ptr %v11)
  br label %bb9
bb7:
  %v21 = extractvalue { i8, { double, double, double }, { double, double, double }, double, double } %v6, 2
  %v22 = extractvalue { i8, { double, double, double }, { double, double, double }, double, double } %v6, 3
  %v23 = extractvalue { double, double, double } %v21, 0
  %v24 = extractvalue { double, double, double } %v21, 1
  %v25 = extractvalue { double, double, double } %v21, 2
  %v26 = call i1 @path_tracer__path_tracer__materials__metal_scatter(double %v23, double %v24, double %v25, double %v22, ptr %v7, ptr %v8, ptr %v9, ptr %v10, ptr %v11)
  br label %bb9
bb8:
  %v27 = extractvalue { i8, { double, double, double }, { double, double, double }, double, double } %v6, 1
  %v28 = extractvalue { double, double, double } %v27, 0
  %v29 = extractvalue { double, double, double } %v27, 1
  %v30 = extractvalue { double, double, double } %v27, 2
  %v31 = call i1 @path_tracer__path_tracer__materials__lambertian_scatter(double %v28, double %v29, double %v30, ptr %v8, ptr %v9, ptr %v10, ptr %v11)
  br label %bb9
bb9:
  %v32 = phi i1 [ %v18, %bb5 ], [ %v20, %bb6 ], [ %v26, %bb7 ], [ %v31, %bb8 ]
  ret i1 %v32
}

define { double, double, double } @path_tracer__path_tracer__vec3__Vec3__to_normalized(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = call double @path_tracer__path_tracer__vec3__Vec3__len(ptr %v1)
  br label %bb1
bb1:
  %v3 = fdiv double 1.0, %v2
  %v4 = load { double, double, double }, ptr %v1
  %v5 = extractvalue { double, double, double } %v4, 0
  %v6 = fmul double %v5, %v3
  %v7 = load { double, double, double }, ptr %v1
  %v8 = extractvalue { double, double, double } %v7, 1
  %v9 = fmul double %v8, %v3
  %v10 = load { double, double, double }, ptr %v1
  %v11 = extractvalue { double, double, double } %v10, 2
  %v12 = fmul double %v11, %v3
  %v13 = insertvalue { double, double, double } undef, double %v6, 0
  %v14 = insertvalue { double, double, double } %v13, double %v9, 1
  %v15 = insertvalue { double, double, double } %v14, double %v12, 2
  ret { double, double, double } %v15
}

define { double, double, double } @path_tracer__path_tracer__camera__random_in_unit_disk(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = call double @randf(ptr %v1)
  br label %bb4
bb1:
  %v3 = phi { double, double, double } [ %v25, %bb5 ], [ %v33, %bb7 ]
  %v4 = extractvalue { double, double, double } %v3, 0
  %v5 = extractvalue { double, double, double } %v3, 0
  %v6 = fmul double %v4, %v5
  %v7 = extractvalue { double, double, double } %v3, 1
  %v8 = extractvalue { double, double, double } %v3, 1
  %v9 = fmul double %v7, %v8
  %v10 = fadd double %v6, %v9
  %v11 = extractvalue { double, double, double } %v3, 2
  %v12 = extractvalue { double, double, double } %v3, 2
  %v13 = fmul double %v11, %v12
  %v14 = fadd double %v10, %v13
  %v15 = fcmp oge double %v14, 1.0
  %v16 = xor i1 %v15, 1
  br i1 %v16, label %bb3, label %bb2
bb2:
  %v17 = call double @randf(ptr %v1)
  br label %bb6
bb3:
  ret { double, double, double } %v3
bb4:
  %v18 = call double @randf(ptr %v1)
  br label %bb5
bb5:
  %v19 = fmul double %v2, 2.0
  %v20 = fmul double %v18, 2.0
  %v21 = fsub double %v19, 1.0
  %v22 = fsub double %v20, 1.0
  %v23 = insertvalue { double, double, double } undef, double %v21, 0
  %v24 = insertvalue { double, double, double } %v23, double %v22, 1
  %v25 = insertvalue { double, double, double } %v24, double 0.0, 2
  br label %bb1
bb6:
  %v26 = call double @randf(ptr %v1)
  br label %bb7
bb7:
  %v27 = fmul double %v17, 2.0
  %v28 = fmul double %v26, 2.0
  %v29 = fsub double %v27, 1.0
  %v30 = fsub double %v28, 1.0
  %v31 = insertvalue { double, double, double } undef, double %v29, 0
  %v32 = insertvalue { double, double, double } %v31, double %v30, 1
  %v33 = insertvalue { double, double, double } %v32, double 0.0, 2
  br label %bb1
}

define i1 @path_tracer__path_tracer__geometry__Triangle__hit(ptr %v0, ptr %v1, double %v2, double %v3, ptr %v4) {
entry:
  br label %bb0
bb0:
  %v5 = phi ptr [ %v0, %entry ]
  %v6 = phi ptr [ %v1, %entry ]
  %v7 = phi double [ %v2, %entry ]
  %v8 = phi double [ %v3, %entry ]
  %v9 = phi ptr [ %v4, %entry ]
  %v10 = alloca { double, double, double }
  %v11 = getelementptr inbounds { { double, double, double }, { double, double, double } }, ptr %v6, i32 0, i32 1
  %v12 = call { double, double, double } @path_tracer__path_tracer__vec3__Vec3__to_normalized(ptr %v11)
  br label %bb1
bb1:
  %v13 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v14 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v13, 4
  %v15 = extractvalue { double, double, double } %v14, 0
  %v16 = extractvalue { double, double, double } %v12, 0
  %v17 = fmul double %v15, %v16
  %v18 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v19 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v18, 4
  %v20 = extractvalue { double, double, double } %v19, 1
  %v21 = extractvalue { double, double, double } %v12, 1
  %v22 = fmul double %v20, %v21
  %v23 = fadd double %v17, %v22
  %v24 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v25 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v24, 4
  %v26 = extractvalue { double, double, double } %v25, 2
  %v27 = extractvalue { double, double, double } %v12, 2
  %v28 = fmul double %v26, %v27
  %v29 = fadd double %v23, %v28
  %v30 = fcmp oeq double %v29, 0.0
  %v31 = xor i1 %v30, 1
  br i1 %v31, label %bb3, label %bb2
bb2:
  br label %bb12
bb3:
  %v32 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v33 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v32, 1
  %v34 = extractvalue { double, double, double } %v33, 0
  %v35 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v36 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v35, 1
  %v37 = extractvalue { double, double, double } %v36, 1
  %v38 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v39 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v38, 1
  %v40 = extractvalue { double, double, double } %v39, 2
  %v41 = load { { double, double, double }, { double, double, double } }, ptr %v6
  %v42 = extractvalue { { double, double, double }, { double, double, double } } %v41, 0
  %v43 = extractvalue { double, double, double } %v42, 0
  %v44 = load { { double, double, double }, { double, double, double } }, ptr %v6
  %v45 = extractvalue { { double, double, double }, { double, double, double } } %v44, 0
  %v46 = extractvalue { double, double, double } %v45, 1
  %v47 = load { { double, double, double }, { double, double, double } }, ptr %v6
  %v48 = extractvalue { { double, double, double }, { double, double, double } } %v47, 0
  %v49 = extractvalue { double, double, double } %v48, 2
  %v50 = fsub double %v34, %v43
  %v51 = fsub double %v37, %v46
  %v52 = fsub double %v40, %v49
  %v53 = fmul double %v15, %v50
  %v54 = fmul double %v20, %v51
  %v55 = fadd double %v53, %v54
  %v56 = fmul double %v26, %v52
  %v57 = fadd double %v55, %v56
  %v58 = fdiv double %v57, %v29
  %v59 = fcmp ole double %v58, 0.0
  %v60 = xor i1 %v59, 1
  br i1 %v60, label %bb5, label %bb4
bb4:
  br label %bb12
bb5:
  %v61 = fmul double %v16, %v58
  %v62 = fmul double %v21, %v58
  %v63 = fmul double %v27, %v58
  %v64 = fadd double %v43, %v61
  %v65 = fadd double %v46, %v62
  %v66 = fadd double %v49, %v63
  %v67 = insertvalue { double, double, double } undef, double %v64, 0
  %v68 = insertvalue { double, double, double } %v67, double %v65, 1
  %v69 = insertvalue { double, double, double } %v68, double %v66, 2
  %v70 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v71 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v70, 2
  %v72 = extractvalue { double, double, double } %v71, 0
  %v73 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v74 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v73, 2
  %v75 = extractvalue { double, double, double } %v74, 1
  %v76 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v77 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v76, 2
  %v78 = extractvalue { double, double, double } %v77, 2
  %v79 = fsub double %v34, %v72
  %v80 = fsub double %v37, %v75
  %v81 = fsub double %v40, %v78
  %v82 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v83 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v82, 3
  %v84 = extractvalue { double, double, double } %v83, 0
  %v85 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v86 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v85, 3
  %v87 = extractvalue { double, double, double } %v86, 1
  %v88 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v89 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v88, 3
  %v90 = extractvalue { double, double, double } %v89, 2
  %v91 = fsub double %v34, %v84
  %v92 = fsub double %v37, %v87
  %v93 = fsub double %v40, %v90
  %v94 = fsub double %v34, %v64
  %v95 = fsub double %v37, %v65
  %v96 = fsub double %v40, %v66
  %v97 = fmul double %v79, %v79
  %v98 = fmul double %v80, %v80
  %v99 = fadd double %v97, %v98
  %v100 = fmul double %v81, %v81
  %v101 = fadd double %v99, %v100
  %v102 = fmul double %v91, %v91
  %v103 = fmul double %v92, %v92
  %v104 = fadd double %v102, %v103
  %v105 = fmul double %v93, %v93
  %v106 = fadd double %v104, %v105
  %v107 = fmul double %v101, %v106
  %v108 = fmul double %v79, %v91
  %v109 = fmul double %v80, %v92
  %v110 = fadd double %v108, %v109
  %v111 = fmul double %v81, %v93
  %v112 = fadd double %v110, %v111
  %v113 = fmul double %v112, %v112
  %v114 = fsub double %v107, %v113
  %v115 = fmul double %v79, %v94
  %v116 = fmul double %v80, %v95
  %v117 = fadd double %v115, %v116
  %v118 = fmul double %v81, %v96
  %v119 = fadd double %v117, %v118
  %v120 = fmul double %v119, %v106
  %v121 = fmul double %v91, %v94
  %v122 = fmul double %v92, %v95
  %v123 = fadd double %v121, %v122
  %v124 = fmul double %v93, %v96
  %v125 = fadd double %v123, %v124
  %v126 = fmul double %v112, %v125
  %v127 = fsub double %v120, %v126
  %v128 = fdiv double %v127, %v114
  %v129 = fmul double %v101, %v125
  %v130 = fmul double %v112, %v119
  %v131 = fsub double %v129, %v130
  %v132 = fdiv double %v131, %v114
  %v133 = fcmp oge double %v128, 0.0
  %v134 = xor i1 %v133, 1
  br i1 %v134, label %bb11, label %bb6
bb6:
  %v135 = fcmp oge double %v132, 0.0
  %v136 = xor i1 %v135, 1
  br i1 %v136, label %bb11, label %bb7
bb7:
  %v137 = fadd double %v128, %v132
  %v138 = fcmp ole double %v137, 1.0
  %v139 = xor i1 %v138, 1
  br i1 %v139, label %bb10, label %bb8
bb8:
  %v140 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v141 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v140, 4
  %v142 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v9, i32 0, i32 3
  store { double, double, double } %v141, ptr %v142
  %v143 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v9, i32 0, i32 2
  store { double, double, double } %v69, ptr %v143
  %v144 = fsub double %v43, %v64
  %v145 = fsub double %v46, %v65
  %v146 = fsub double %v49, %v66
  %v147 = insertvalue { double, double, double } undef, double %v144, 0
  %v148 = insertvalue { double, double, double } %v147, double %v145, 1
  %v149 = insertvalue { double, double, double } %v148, double %v146, 2
  store { double, double, double } %v149, ptr %v10
  %v150 = bitcast ptr %v10 to ptr
  %v151 = call double @path_tracer__path_tracer__vec3__Vec3__len(ptr %v150)
  br label %bb9
bb9:
  %v152 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v9, i32 0, i32 1
  store double %v151, ptr %v152
  %v153 = load { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } }, ptr %v5
  %v154 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, { double, double, double }, { double, double, double }, { double, double, double }, { double, double, double } } %v153, 0
  %v155 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v9, i32 0, i32 0
  store { i8, { double, double, double }, { double, double, double }, double, double } %v154, ptr %v155
  br label %bb12
bb10:
  br label %bb11
bb11:
  br label %bb13
bb12:
  %v156 = phi i1 [ 0, %bb2 ], [ 0, %bb4 ], [ 1, %bb9 ]
  br label %bb13
bb13:
  %v157 = phi i1 [ 0, %bb11 ], [ %v156, %bb12 ]
  ret i1 %v157
}

define i1 @path_tracer__path_tracer__materials__normal_scatter(ptr %v0, ptr %v1, ptr %v2, ptr %v3) {
entry:
  br label %bb0
bb0:
  %v4 = phi ptr [ %v0, %entry ]
  %v5 = phi ptr [ %v1, %entry ]
  %v6 = phi ptr [ %v2, %entry ]
  %v7 = phi ptr [ %v3, %entry ]
  %v8 = alloca { double, double, double }
  %v9 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v10 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v9, 2
  %v11 = extractvalue { double, double, double } %v10, 0
  %v12 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v13 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v12, 2
  %v14 = extractvalue { double, double, double } %v13, 1
  %v15 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v16 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v15, 2
  %v17 = extractvalue { double, double, double } %v16, 2
  %v18 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v19 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v18, 3
  %v20 = extractvalue { double, double, double } %v19, 0
  %v21 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v22 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v21, 3
  %v23 = extractvalue { double, double, double } %v22, 1
  %v24 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v25 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v24, 3
  %v26 = extractvalue { double, double, double } %v25, 2
  %v27 = fadd double %v11, %v20
  %v28 = fadd double %v14, %v23
  %v29 = fadd double %v17, %v26
  %v30 = call { double, double, double } @path_tracer__path_tracer__materials__random_in_unit_sphere(ptr %v7)
  br label %bb1
bb1:
  %v31 = extractvalue { double, double, double } %v30, 0
  %v32 = fadd double %v27, %v31
  %v33 = extractvalue { double, double, double } %v30, 1
  %v34 = fadd double %v28, %v33
  %v35 = extractvalue { double, double, double } %v30, 2
  %v36 = fadd double %v29, %v35
  %v37 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4
  %v38 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v37, 2
  %v39 = fsub double %v32, %v11
  %v40 = fsub double %v34, %v14
  %v41 = fsub double %v36, %v17
  %v42 = insertvalue { double, double, double } undef, double %v39, 0
  %v43 = insertvalue { double, double, double } %v42, double %v40, 1
  %v44 = insertvalue { double, double, double } %v43, double %v41, 2
  %v45 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v38, 0
  %v46 = insertvalue { { double, double, double }, { double, double, double } } %v45, { double, double, double } %v44, 1
  store { { double, double, double }, { double, double, double } } %v46, ptr %v6
  %v47 = getelementptr inbounds { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v4, i32 0, i32 3
  %v48 = call { double, double, double } @path_tracer__path_tracer__vec3__Vec3__to_normalized(ptr %v47)
  store { double, double, double } %v48, ptr %v8
  br label %bb2
bb2:
  %v49 = load { double, double, double }, ptr %v8
  %v50 = extractvalue { double, double, double } %v49, 0
  %v51 = fcmp olt double %v50, 0.0
  %v52 = xor i1 %v51, 1
  br i1 %v52, label %bb4, label %bb3
bb3:
  %v53 = load { double, double, double }, ptr %v8
  %v54 = extractvalue { double, double, double } %v53, 0
  %v55 = fneg double %v54
  %v56 = getelementptr inbounds { double, double, double }, ptr %v8, i32 0, i32 0
  store double %v55, ptr %v56
  br label %bb5
bb4:
  br label %bb5
bb5:
  %v57 = load { double, double, double }, ptr %v8
  %v58 = extractvalue { double, double, double } %v57, 1
  %v59 = fcmp olt double %v58, 0.0
  %v60 = xor i1 %v59, 1
  br i1 %v60, label %bb7, label %bb6
bb6:
  %v61 = load { double, double, double }, ptr %v8
  %v62 = extractvalue { double, double, double } %v61, 1
  %v63 = fneg double %v62
  %v64 = getelementptr inbounds { double, double, double }, ptr %v8, i32 0, i32 1
  store double %v63, ptr %v64
  br label %bb8
bb7:
  br label %bb8
bb8:
  %v65 = load { double, double, double }, ptr %v8
  %v66 = extractvalue { double, double, double } %v65, 2
  %v67 = fcmp olt double %v66, 0.0
  %v68 = xor i1 %v67, 1
  br i1 %v68, label %bb10, label %bb9
bb9:
  %v69 = load { double, double, double }, ptr %v8
  %v70 = extractvalue { double, double, double } %v69, 2
  %v71 = fneg double %v70
  %v72 = getelementptr inbounds { double, double, double }, ptr %v8, i32 0, i32 2
  store double %v71, ptr %v72
  br label %bb11
bb10:
  br label %bb11
bb11:
  %v73 = load { double, double, double }, ptr %v8
  store { double, double, double } %v73, ptr %v5
  ret i1 1
}

define i1 @path_tracer__path_tracer__materials__dielectric_scatter(double %v0, ptr %v1, ptr %v2, ptr %v3, ptr %v4, ptr %v5) {
entry:
  br label %bb0
bb0:
  %v6 = phi double [ %v0, %entry ]
  %v7 = phi ptr [ %v1, %entry ]
  %v8 = phi ptr [ %v2, %entry ]
  %v9 = phi ptr [ %v3, %entry ]
  %v10 = phi ptr [ %v4, %entry ]
  %v11 = phi ptr [ %v5, %entry ]
  %v12 = alloca double
  %v13 = alloca { double, double, double }
  %v14 = alloca double
  %v15 = alloca double
  %v16 = alloca { double, double, double }
  store double %v6, ptr %v12
  %v17 = load { { double, double, double }, { double, double, double } }, ptr %v7
  %v18 = extractvalue { { double, double, double }, { double, double, double } } %v17, 1
  %v19 = extractvalue { double, double, double } %v18, 0
  %v20 = load { { double, double, double }, { double, double, double } }, ptr %v7
  %v21 = extractvalue { { double, double, double }, { double, double, double } } %v20, 1
  %v22 = extractvalue { double, double, double } %v21, 1
  %v23 = load { { double, double, double }, { double, double, double } }, ptr %v7
  %v24 = extractvalue { { double, double, double }, { double, double, double } } %v23, 1
  %v25 = extractvalue { double, double, double } %v24, 2
  %v26 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v8
  %v27 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v26, 3
  %v28 = extractvalue { double, double, double } %v27, 0
  %v29 = fmul double %v19, %v28
  %v30 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v8
  %v31 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v30, 3
  %v32 = extractvalue { double, double, double } %v31, 1
  %v33 = fmul double %v22, %v32
  %v34 = fadd double %v29, %v33
  %v35 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v8
  %v36 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v35, 3
  %v37 = extractvalue { double, double, double } %v36, 2
  %v38 = fmul double %v25, %v37
  %v39 = fadd double %v34, %v38
  %v40 = fmul double 2.0, %v39
  %v41 = fmul double %v28, %v40
  %v42 = fmul double %v32, %v40
  %v43 = fmul double %v37, %v40
  %v44 = fsub double %v19, %v41
  %v45 = fsub double %v22, %v42
  %v46 = fsub double %v25, %v43
  %v47 = insertvalue { double, double, double } undef, double %v44, 0
  %v48 = insertvalue { double, double, double } %v47, double %v45, 1
  %v49 = insertvalue { double, double, double } %v48, double %v46, 2
  %v50 = insertvalue { double, double, double } undef, double 0.0, 0
  %v51 = insertvalue { double, double, double } %v50, double 0.0, 1
  %v52 = insertvalue { double, double, double } %v51, double 0.0, 2
  store { double, double, double } %v52, ptr %v16
  %v53 = insertvalue { double, double, double } undef, double 1.0, 0
  %v54 = insertvalue { double, double, double } %v53, double 1.0, 1
  %v55 = insertvalue { double, double, double } %v54, double 1.0, 2
  store { double, double, double } %v55, ptr %v9
  %v56 = fcmp ogt double %v39, 0.0
  %v57 = xor i1 %v56, 1
  br i1 %v57, label %bb3, label %bb1
bb1:
  %v58 = fneg double %v28
  %v59 = fneg double %v32
  %v60 = fneg double %v37
  %v61 = insertvalue { double, double, double } undef, double %v58, 0
  %v62 = insertvalue { double, double, double } %v61, double %v59, 1
  %v63 = insertvalue { double, double, double } %v62, double %v60, 2
  store { double, double, double } %v63, ptr %v13
  %v64 = load double, ptr %v12
  store double %v64, ptr %v14
  %v65 = load double, ptr %v12
  %v66 = fmul double %v65, %v39
  %v67 = getelementptr inbounds { { double, double, double }, { double, double, double } }, ptr %v7, i32 0, i32 1
  %v68 = call double @path_tracer__path_tracer__vec3__Vec3__len(ptr %v67)
  br label %bb2
bb2:
  %v69 = fdiv double %v66, %v68
  store double %v69, ptr %v15
  br label %bb5
bb3:
  %v70 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v8
  %v71 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v70, 3
  store { double, double, double } %v71, ptr %v13
  %v72 = load double, ptr %v12
  %v73 = fdiv double 1.0, %v72
  store double %v73, ptr %v14
  %v74 = fneg double %v39
  %v75 = getelementptr inbounds { { double, double, double }, { double, double, double } }, ptr %v7, i32 0, i32 1
  %v76 = call double @path_tracer__path_tracer__vec3__Vec3__len(ptr %v75)
  br label %bb4
bb4:
  %v77 = fdiv double %v74, %v76
  store double %v77, ptr %v15
  br label %bb5
bb5:
  %v78 = getelementptr inbounds { { double, double, double }, { double, double, double } }, ptr %v7, i32 0, i32 1
  %v79 = bitcast ptr %v13 to ptr
  %v80 = bitcast ptr %v14 to ptr
  %v81 = call i1 @path_tracer__path_tracer__materials__refract(ptr %v78, ptr %v79, ptr %v80, ptr %v16)
  br label %bb6
bb6:
  %v82 = xor i1 %v81, 1
  br i1 %v82, label %bb9, label %bb7
bb7:
  %v83 = bitcast ptr %v15 to ptr
  %v84 = bitcast ptr %v12 to ptr
  %v85 = call double @path_tracer__path_tracer__materials__schlick(ptr %v83, ptr %v84)
  br label %bb8
bb8:
  br label %bb10
bb9:
  br label %bb10
bb10:
  %v86 = phi double [ %v85, %bb8 ], [ 1.0, %bb9 ]
  %v87 = call double @randf(ptr %v11)
  br label %bb14
bb11:
  %v88 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v8
  %v89 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v88, 2
  %v90 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v89, 0
  %v91 = insertvalue { { double, double, double }, { double, double, double } } %v90, { double, double, double } %v49, 1
  store { { double, double, double }, { double, double, double } } %v91, ptr %v10
  br label %bb13
bb12:
  %v92 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v8
  %v93 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v92, 2
  %v94 = load { double, double, double }, ptr %v16
  %v95 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v93, 0
  %v96 = insertvalue { { double, double, double }, { double, double, double } } %v95, { double, double, double } %v94, 1
  store { { double, double, double }, { double, double, double } } %v96, ptr %v10
  br label %bb13
bb13:
  ret i1 1
bb14:
  %v97 = fcmp olt double %v87, %v86
  %v98 = xor i1 %v97, 1
  br i1 %v98, label %bb12, label %bb11
}

define i1 @path_tracer__path_tracer__materials__metal_scatter(double %v0, double %v1, double %v2, double %v3, ptr %v4, ptr %v5, ptr %v6, ptr %v7, ptr %v8) {
entry:
  %v9 = insertvalue { double, double, double } undef, double %v0, 0
  %v10 = insertvalue { double, double, double } %v9, double %v1, 1
  %v11 = insertvalue { double, double, double } %v10, double %v2, 2
  br label %bb0
bb0:
  %v12 = phi { double, double, double } [ %v11, %entry ]
  %v13 = phi double [ %v3, %entry ]
  %v14 = phi ptr [ %v4, %entry ]
  %v15 = phi ptr [ %v5, %entry ]
  %v16 = phi ptr [ %v6, %entry ]
  %v17 = phi ptr [ %v7, %entry ]
  %v18 = phi ptr [ %v8, %entry ]
  %v19 = getelementptr inbounds { { double, double, double }, { double, double, double } }, ptr %v14, i32 0, i32 1
  %v20 = call { double, double, double } @path_tracer__path_tracer__vec3__Vec3__to_normalized(ptr %v19)
  br label %bb1
bb1:
  %v21 = extractvalue { double, double, double } %v20, 0
  %v22 = extractvalue { double, double, double } %v20, 1
  %v23 = extractvalue { double, double, double } %v20, 2
  %v24 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  %v25 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v24, 3
  %v26 = extractvalue { double, double, double } %v25, 0
  %v27 = fmul double %v21, %v26
  %v28 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  %v29 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v28, 3
  %v30 = extractvalue { double, double, double } %v29, 1
  %v31 = fmul double %v22, %v30
  %v32 = fadd double %v27, %v31
  %v33 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  %v34 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v33, 3
  %v35 = extractvalue { double, double, double } %v34, 2
  %v36 = fmul double %v23, %v35
  %v37 = fadd double %v32, %v36
  %v38 = fmul double 2.0, %v37
  %v39 = fmul double %v26, %v38
  %v40 = fmul double %v30, %v38
  %v41 = fmul double %v35, %v38
  %v42 = fsub double %v21, %v39
  %v43 = fsub double %v22, %v40
  %v44 = fsub double %v23, %v41
  %v45 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v15
  %v46 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v45, 2
  %v47 = call { double, double, double } @path_tracer__path_tracer__materials__random_in_unit_sphere(ptr %v18)
  br label %bb2
bb2:
  %v48 = extractvalue { double, double, double } %v47, 0
  %v49 = fmul double %v48, %v13
  %v50 = extractvalue { double, double, double } %v47, 1
  %v51 = fmul double %v50, %v13
  %v52 = extractvalue { double, double, double } %v47, 2
  %v53 = fmul double %v52, %v13
  %v54 = fadd double %v42, %v49
  %v55 = fadd double %v43, %v51
  %v56 = fadd double %v44, %v53
  %v57 = insertvalue { double, double, double } undef, double %v54, 0
  %v58 = insertvalue { double, double, double } %v57, double %v55, 1
  %v59 = insertvalue { double, double, double } %v58, double %v56, 2
  %v60 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v46, 0
  %v61 = insertvalue { { double, double, double }, { double, double, double } } %v60, { double, double, double } %v59, 1
  store { { double, double, double }, { double, double, double } } %v61, ptr %v17
  store { double, double, double } %v12, ptr %v16
  %v62 = load { { double, double, double }, { double, double, double } }, ptr %v17
  %v63 = extractvalue { { double, double, double }, { double, double, double } } %v62, 1
  %v64 = extractvalue { double, double, double } %v63, 0
  %v65 = fmul double %v64, %v26
  %v66 = load { { double, double, double }, { double, double, double } }, ptr %v17
  %v67 = extractvalue { { double, double, double }, { double, double, double } } %v66, 1
  %v68 = extractvalue { double, double, double } %v67, 1
  %v69 = fmul double %v68, %v30
  %v70 = fadd double %v65, %v69
  %v71 = load { { double, double, double }, { double, double, double } }, ptr %v17
  %v72 = extractvalue { { double, double, double }, { double, double, double } } %v71, 1
  %v73 = extractvalue { double, double, double } %v72, 2
  %v74 = fmul double %v73, %v35
  %v75 = fadd double %v70, %v74
  %v76 = fcmp ogt double %v75, 0.0
  ret i1 %v76
}

define i1 @path_tracer__path_tracer__materials__lambertian_scatter(double %v0, double %v1, double %v2, ptr %v3, ptr %v4, ptr %v5, ptr %v6) {
entry:
  %v7 = insertvalue { double, double, double } undef, double %v0, 0
  %v8 = insertvalue { double, double, double } %v7, double %v1, 1
  %v9 = insertvalue { double, double, double } %v8, double %v2, 2
  br label %bb0
bb0:
  %v10 = phi { double, double, double } [ %v9, %entry ]
  %v11 = phi ptr [ %v3, %entry ]
  %v12 = phi ptr [ %v4, %entry ]
  %v13 = phi ptr [ %v5, %entry ]
  %v14 = phi ptr [ %v6, %entry ]
  %v15 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v16 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v15, 2
  %v17 = extractvalue { double, double, double } %v16, 0
  %v18 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v19 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v18, 2
  %v20 = extractvalue { double, double, double } %v19, 1
  %v21 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v22 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v21, 2
  %v23 = extractvalue { double, double, double } %v22, 2
  %v24 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v25 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v24, 3
  %v26 = extractvalue { double, double, double } %v25, 0
  %v27 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v28 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v27, 3
  %v29 = extractvalue { double, double, double } %v28, 1
  %v30 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v31 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v30, 3
  %v32 = extractvalue { double, double, double } %v31, 2
  %v33 = fadd double %v17, %v26
  %v34 = fadd double %v20, %v29
  %v35 = fadd double %v23, %v32
  %v36 = call { double, double, double } @path_tracer__path_tracer__materials__random_in_unit_sphere(ptr %v14)
  br label %bb1
bb1:
  %v37 = extractvalue { double, double, double } %v36, 0
  %v38 = fadd double %v33, %v37
  %v39 = extractvalue { double, double, double } %v36, 1
  %v40 = fadd double %v34, %v39
  %v41 = extractvalue { double, double, double } %v36, 2
  %v42 = fadd double %v35, %v41
  %v43 = load { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } }, ptr %v11
  %v44 = extractvalue { { i8, { double, double, double }, { double, double, double }, double, double }, double, { double, double, double }, { double, double, double } } %v43, 2
  %v45 = fsub double %v38, %v17
  %v46 = fsub double %v40, %v20
  %v47 = fsub double %v42, %v23
  %v48 = insertvalue { double, double, double } undef, double %v45, 0
  %v49 = insertvalue { double, double, double } %v48, double %v46, 1
  %v50 = insertvalue { double, double, double } %v49, double %v47, 2
  %v51 = insertvalue { { double, double, double }, { double, double, double } } undef, { double, double, double } %v44, 0
  %v52 = insertvalue { { double, double, double }, { double, double, double } } %v51, { double, double, double } %v50, 1
  store { { double, double, double }, { double, double, double } } %v52, ptr %v13
  store { double, double, double } %v10, ptr %v12
  ret i1 1
}

define double @path_tracer__path_tracer__vec3__Vec3__len(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = load { double, double, double }, ptr %v1
  %v3 = extractvalue { double, double, double } %v2, 0
  %v4 = fmul double %v3, %v3
  %v5 = load { double, double, double }, ptr %v1
  %v6 = extractvalue { double, double, double } %v5, 1
  %v7 = fmul double %v6, %v6
  %v8 = fadd double %v4, %v7
  %v9 = load { double, double, double }, ptr %v1
  %v10 = extractvalue { double, double, double } %v9, 2
  %v11 = fmul double %v10, %v10
  %v12 = fadd double %v8, %v11
  %v13 = call double @sqrt_f64(double %v12)
  br label %bb1
bb1:
  ret double %v13
}

define { double, double, double } @path_tracer__path_tracer__materials__random_in_unit_sphere(ptr %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = call double @randf(ptr %v1)
  br label %bb4
bb1:
  %v3 = phi { double, double, double } [ %v25, %bb6 ], [ %v36, %bb9 ]
  %v4 = extractvalue { double, double, double } %v3, 0
  %v5 = fmul double %v4, %v4
  %v6 = extractvalue { double, double, double } %v3, 1
  %v7 = fmul double %v6, %v6
  %v8 = fadd double %v5, %v7
  %v9 = extractvalue { double, double, double } %v3, 2
  %v10 = fmul double %v9, %v9
  %v11 = fadd double %v8, %v10
  %v12 = fcmp oge double %v11, 1.0
  %v13 = xor i1 %v12, 1
  br i1 %v13, label %bb3, label %bb2
bb2:
  %v14 = call double @randf(ptr %v1)
  br label %bb7
bb3:
  ret { double, double, double } %v3
bb4:
  %v15 = call double @randf(ptr %v1)
  br label %bb5
bb5:
  %v16 = call double @randf(ptr %v1)
  br label %bb6
bb6:
  %v17 = fmul double %v2, 2.0
  %v18 = fmul double %v15, 2.0
  %v19 = fmul double %v16, 2.0
  %v20 = fsub double %v17, 1.0
  %v21 = fsub double %v18, 1.0
  %v22 = fsub double %v19, 1.0
  %v23 = insertvalue { double, double, double } undef, double %v20, 0
  %v24 = insertvalue { double, double, double } %v23, double %v21, 1
  %v25 = insertvalue { double, double, double } %v24, double %v22, 2
  br label %bb1
bb7:
  %v26 = call double @randf(ptr %v1)
  br label %bb8
bb8:
  %v27 = call double @randf(ptr %v1)
  br label %bb9
bb9:
  %v28 = fmul double %v14, 2.0
  %v29 = fmul double %v26, 2.0
  %v30 = fmul double %v27, 2.0
  %v31 = fsub double %v28, 1.0
  %v32 = fsub double %v29, 1.0
  %v33 = fsub double %v30, 1.0
  %v34 = insertvalue { double, double, double } undef, double %v31, 0
  %v35 = insertvalue { double, double, double } %v34, double %v32, 1
  %v36 = insertvalue { double, double, double } %v35, double %v33, 2
  br label %bb1
}

define i1 @path_tracer__path_tracer__materials__refract(ptr %v0, ptr %v1, ptr %v2, ptr %v3) {
entry:
  br label %bb0
bb0:
  %v4 = phi ptr [ %v0, %entry ]
  %v5 = phi ptr [ %v1, %entry ]
  %v6 = phi ptr [ %v2, %entry ]
  %v7 = phi ptr [ %v3, %entry ]
  %v8 = call { double, double, double } @path_tracer__path_tracer__vec3__Vec3__to_normalized(ptr %v4)
  br label %bb1
bb1:
  %v9 = extractvalue { double, double, double } %v8, 0
  %v10 = load { double, double, double }, ptr %v5
  %v11 = extractvalue { double, double, double } %v10, 0
  %v12 = fmul double %v9, %v11
  %v13 = extractvalue { double, double, double } %v8, 1
  %v14 = load { double, double, double }, ptr %v5
  %v15 = extractvalue { double, double, double } %v14, 1
  %v16 = fmul double %v13, %v15
  %v17 = fadd double %v12, %v16
  %v18 = extractvalue { double, double, double } %v8, 2
  %v19 = load { double, double, double }, ptr %v5
  %v20 = extractvalue { double, double, double } %v19, 2
  %v21 = fmul double %v18, %v20
  %v22 = fadd double %v17, %v21
  %v23 = load double, ptr %v6
  %v24 = fmul double %v23, %v23
  %v25 = fmul double %v22, %v22
  %v26 = fsub double 1.0, %v25
  %v27 = fmul double %v24, %v26
  %v28 = fsub double 1.0, %v27
  %v29 = fcmp ogt double %v28, 0.0
  %v30 = xor i1 %v29, 1
  br i1 %v30, label %bb3, label %bb2
bb2:
  %v31 = fmul double %v11, %v22
  %v32 = fmul double %v15, %v22
  %v33 = fmul double %v20, %v22
  %v34 = fsub double %v9, %v31
  %v35 = fsub double %v13, %v32
  %v36 = fsub double %v18, %v33
  %v37 = fmul double %v34, %v23
  %v38 = fmul double %v35, %v23
  %v39 = fmul double %v36, %v23
  %v40 = call double @sqrt_f64(double %v28)
  br label %bb5
bb3:
  br label %bb4
bb4:
  %v41 = phi i1 [ 0, %bb3 ], [ 1, %bb5 ]
  ret i1 %v41
bb5:
  %v42 = fmul double %v11, %v40
  %v43 = fmul double %v15, %v40
  %v44 = fmul double %v20, %v40
  %v45 = fsub double %v37, %v42
  %v46 = fsub double %v38, %v43
  %v47 = fsub double %v39, %v44
  %v48 = insertvalue { double, double, double } undef, double %v45, 0
  %v49 = insertvalue { double, double, double } %v48, double %v46, 1
  %v50 = insertvalue { double, double, double } %v49, double %v47, 2
  store { double, double, double } %v50, ptr %v7
  br label %bb4
}

define double @path_tracer__path_tracer__materials__schlick(ptr %v0, ptr %v1) {
entry:
  br label %bb0
bb0:
  %v2 = phi ptr [ %v0, %entry ]
  %v3 = phi ptr [ %v1, %entry ]
  %v4 = load double, ptr %v3
  %v5 = fsub double 1.0, %v4
  %v6 = fadd double 1.0, %v4
  %v7 = fdiv double %v5, %v6
  %v8 = fmul double %v7, %v7
  %v9 = fsub double 1.0, %v8
  %v10 = load double, ptr %v2
  %v11 = fsub double 1.0, %v10
  %v12 = call double @pow_f64(double %v11, double 5.0)
  br label %bb1
bb1:
  %v13 = fmul double %v9, %v12
  %v14 = fadd double %v8, %v13
  ret double %v14
}

define double @pow_f64(double %v0, double %v1) {
entry:
  br label %bb0
bb0:
  %v2 = phi double [ %v0, %entry ]
  %v3 = phi double [ %v1, %entry ]
  %v4 = fcmp oeq double %v2, 0.0
  %v5 = xor i1 %v4, 1
  br i1 %v5, label %bb2, label %bb1
bb1:
  br label %bb5
bb2:
  %v6 = fcmp oeq double %v3, 0.0
  %v7 = xor i1 %v6, 1
  br i1 %v7, label %bb4, label %bb3
bb3:
  br label %bb5
bb4:
  %v8 = call double @log_f64(double %v2)
  br label %bb6
bb5:
  %v9 = phi double [ 0.0, %bb1 ], [ 1.0, %bb3 ], [ %v11, %bb7 ]
  ret double %v9
bb6:
  %v10 = fmul double %v3, %v8
  %v11 = call double @exp_f64(double %v10)
  br label %bb7
bb7:
  br label %bb5
}

define double @log_f64(double %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi double [ %v0, %entry ]
  %v2 = fcmp ole double %v1, 0.0
  %v3 = xor i1 %v2, 1
  br i1 %v3, label %bb2, label %bb1
bb1:
  br label %bb13
bb2:
  br label %bb3
bb3:
  %v4 = phi double [ %v1, %bb2 ], [ %v8, %bb4 ]
  %v5 = phi i32 [ 0, %bb2 ], [ %v9, %bb4 ]
  %v6 = fcmp ogt double %v4, 1.0
  %v7 = xor i1 %v6, 1
  br i1 %v7, label %bb5, label %bb4
bb4:
  %v8 = fmul double %v4, 2.0
  %v9 = add i32 %v5, 1
  br label %bb3
bb5:
  br label %bb6
bb6:
  %v10 = phi double [ %v4, %bb5 ], [ %v14, %bb7 ]
  %v11 = phi i32 [ %v5, %bb5 ], [ %v15, %bb7 ]
  %v12 = fcmp olt double %v10, 0.5
  %v13 = xor i1 %v12, 1
  br i1 %v13, label %bb8, label %bb7
bb7:
  %v14 = fmul double %v10, 2.0
  %v15 = sub i32 %v11, 1
  br label %bb6
bb8:
  %v16 = fsub double %v10, 1.0
  br label %bb9
bb9:
  %v17 = phi double [ %v16, %bb8 ], [ %v28, %bb11 ]
  %v18 = phi double [ %v16, %bb8 ], [ %v24, %bb11 ]
  %v19 = phi i32 [ 0, %bb8 ], [ %v41, %bb11 ]
  %v20 = icmp slt i32 %v19, 20
  %v21 = xor i1 %v20, 1
  br i1 %v21, label %bb15, label %bb14
bb10:
  unreachable
bb11:
  %v22 = extractvalue { i8, i32 } %v40, 1
  %v23 = fneg double %v16
  %v24 = fmul double %v18, %v23
  %v25 = sitofp i32 %v22 to double
  %v26 = fadd double %v25, 1.0
  %v27 = fdiv double %v24, %v26
  %v28 = fadd double %v17, %v27
  br label %bb9
bb12:
  %v29 = sitofp i32 %v11 to double
  %v30 = fmul double %v29, 0.6931471805599453
  %v31 = fadd double %v17, %v30
  br label %bb13
bb13:
  %v32 = phi double [ 0.0, %bb1 ], [ %v31, %bb12 ]
  ret double %v32
bb14:
  %v33 = add i32 %v19, 1
  %v34 = insertvalue { i32, i1 } undef, i32 %v33, 0
  %v35 = insertvalue { i32, i1 } %v34, i1 0, 1
  %v36 = extractvalue { i32, i1 } %v35, 0
  %v37 = extractvalue { i32, i1 } %v35, 1
  %v38 = xor i1 %v37, 1
  br i1 %v38, label %bb19, label %bb18
bb15:
  %v39 = insertvalue { i8, i32 } undef, i8 0, 0
  br label %bb16
bb16:
  %v40 = phi { i8, i32 } [ %v39, %bb15 ], [ %v47, %bb19 ]
  %v41 = phi i32 [ %v19, %bb15 ], [ %v36, %bb19 ]
  %v42 = extractvalue { i8, i32 } %v40, 0
  %v43 = zext i8 %v42 to i64
  %v44 = icmp eq i64 %v43, 0
  br i1 %v44, label %bb12, label %bb17
bb17:
  %v45 = icmp eq i64 %v43, 1
  br i1 %v45, label %bb11, label %bb10
bb18:
  br label %bb10
bb19:
  %v46 = insertvalue { i8, i32 } undef, i8 1, 0
  %v47 = insertvalue { i8, i32 } %v46, i32 %v19, 1
  br label %bb16
}

declare i32 @llvm.fptosi.sat.i32.f64(double)

define double @exp_f64(double %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi double [ %v0, %entry ]
  %v2 = fcmp oeq double %v1, 0.0
  %v3 = xor i1 %v2, 1
  br i1 %v3, label %bb2, label %bb1
bb1:
  br label %bb7
bb2:
  %v4 = fdiv double %v1, 0.6931471805599453
  %v5 = call double @floor_f64(double %v4)
  br label %bb8
bb3:
  %v6 = phi double [ %v15, %bb5 ], [ 1.0, %bb8 ]
  %v7 = phi double [ %v14, %bb5 ], [ 1.0, %bb8 ]
  %v8 = phi i32 [ %v30, %bb5 ], [ 0, %bb8 ]
  %v9 = icmp slt i32 %v8, 20
  %v10 = xor i1 %v9, 1
  br i1 %v10, label %bb10, label %bb9
bb4:
  unreachable
bb5:
  %v11 = extractvalue { i8, i32 } %v29, 1
  %v12 = sitofp i32 %v11 to double
  %v13 = fdiv double %v21, %v12
  %v14 = fmul double %v7, %v13
  %v15 = fmul double %v6, %v14
  br label %bb3
bb6:
  %v16 = call double @powi_f64(double 2.0, i32 %v18)
  br label %bb15
bb7:
  %v17 = phi double [ 1.0, %bb1 ], [ %v37, %bb15 ]
  ret double %v17
bb8:
  %v18 = call i32 @llvm.fptosi.sat.i32.f64(double %v5)
  %v19 = sitofp i32 %v18 to double
  %v20 = fmul double %v19, 0.6931471805599453
  %v21 = fsub double %v1, %v20
  br label %bb3
bb9:
  %v22 = add i32 %v8, 1
  %v23 = insertvalue { i32, i1 } undef, i32 %v22, 0
  %v24 = insertvalue { i32, i1 } %v23, i1 0, 1
  %v25 = extractvalue { i32, i1 } %v24, 0
  %v26 = extractvalue { i32, i1 } %v24, 1
  %v27 = xor i1 %v26, 1
  br i1 %v27, label %bb14, label %bb13
bb10:
  %v28 = insertvalue { i8, i32 } undef, i8 0, 0
  br label %bb11
bb11:
  %v29 = phi { i8, i32 } [ %v28, %bb10 ], [ %v36, %bb14 ]
  %v30 = phi i32 [ %v8, %bb10 ], [ %v25, %bb14 ]
  %v31 = extractvalue { i8, i32 } %v29, 0
  %v32 = zext i8 %v31 to i64
  %v33 = icmp eq i64 %v32, 0
  br i1 %v33, label %bb6, label %bb12
bb12:
  %v34 = icmp eq i64 %v32, 1
  br i1 %v34, label %bb5, label %bb4
bb13:
  br label %bb4
bb14:
  %v35 = insertvalue { i8, i32 } undef, i8 1, 0
  %v36 = insertvalue { i8, i32 } %v35, i32 %v8, 1
  br label %bb11
bb15:
  %v37 = fmul double %v6, %v16
  br label %bb7
}

declare i64 @llvm.fptosi.sat.i64.f64(double)

define double @floor_f64(double %v0) {
entry:
  br label %bb0
bb0:
  %v1 = phi double [ %v0, %entry ]
  %v2 = call i64 @llvm.fptosi.sat.i64.f64(double %v1)
  %v3 = sitofp i64 %v2 to double
  ret double %v3
}

define double @powi_f64(double %v0, i32 %v1) {
entry:
  br label %bb0
bb0:
  %v2 = phi double [ %v0, %entry ]
  %v3 = phi i32 [ %v1, %entry ]
  %v4 = icmp eq i32 %v3, 0
  br i1 %v4, label %bb1, label %bb2
bb1:
  br label %bb12
bb2:
  %v5 = icmp slt i32 %v3, 0
  %v6 = xor i1 %v5, 1
  br i1 %v6, label %bb4, label %bb3
bb3:
  %v7 = sub i32 0, %v3
  %v8 = call double @path_tracer__path_tracer__util__powi_f64(double %v2, i32 %v7)
  br label %bb5
bb4:
  %v9 = bitcast i32 %v3 to i32
  br label %bb6
bb5:
  %v10 = fdiv double 1.0, %v8
  br label %bb12
bb6:
  %v11 = phi double [ 1.0, %bb4 ], [ %v19, %bb10 ]
  %v12 = phi double [ %v2, %bb4 ], [ %v20, %bb10 ]
  %v13 = phi i32 [ %v9, %bb4 ], [ %v21, %bb10 ]
  %v14 = icmp ugt i32 %v13, 0
  %v15 = xor i1 %v14, 1
  br i1 %v15, label %bb11, label %bb7
bb7:
  %v16 = urem i32 %v13, 2
  %v17 = icmp eq i32 %v16, 1
  br i1 %v17, label %bb8, label %bb9
bb8:
  %v18 = fmul double %v11, %v12
  br label %bb10
bb9:
  br label %bb10
bb10:
  %v19 = phi double [ %v18, %bb8 ], [ %v11, %bb9 ]
  %v20 = fmul double %v12, %v12
  %v21 = udiv i32 %v13, 2
  br label %bb6
bb11:
  br label %bb12
bb12:
  %v22 = phi double [ 1.0, %bb1 ], [ %v10, %bb5 ], [ %v11, %bb11 ]
  ret double %v22
}

define double @path_tracer__path_tracer__util__powi_f64(double %v0, i32 %v1) {
entry:
  br label %bb0
bb0:
  %v2 = phi double [ %v0, %entry ]
  %v3 = phi i32 [ %v1, %entry ]
  %v4 = call double @powi_f64(double %v2, i32 %v3)
  br label %bb1
bb1:
  ret double %v4
}

