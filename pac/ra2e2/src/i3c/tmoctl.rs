#[doc = "Register `TMOCTL` reader"]
pub struct R(crate::R<TMOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMOCTL` writer"]
pub struct W(crate::W<TMOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TMOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TODTS` reader - Timeout Detection Time Selection"]
pub type TODTS_R = crate::FieldReader<u8, TODTS_A>;
#[doc = "Timeout Detection Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TODTS_A {
    #[doc = "0: 16bit-timeout"]
    _00 = 0,
    #[doc = "1: 14bit-timeout"]
    _01 = 1,
    #[doc = "2: 8bit-timeout"]
    _10 = 2,
    #[doc = "3: 6bit-timeout"]
    _11 = 3,
}
impl From<TODTS_A> for u8 {
    #[inline(always)]
    fn from(variant: TODTS_A) -> Self {
        variant as _
    }
}
impl TODTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TODTS_A {
        match self.bits {
            0 => TODTS_A::_00,
            1 => TODTS_A::_01,
            2 => TODTS_A::_10,
            3 => TODTS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TODTS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TODTS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TODTS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TODTS_A::_11
    }
}
#[doc = "Field `TODTS` writer - Timeout Detection Time Selection"]
pub type TODTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TMOCTL_SPEC, u8, TODTS_A, 2, O>;
impl<'a, const O: u8> TODTS_W<'a, O> {
    #[doc = "16bit-timeout"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TODTS_A::_00)
    }
    #[doc = "14bit-timeout"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TODTS_A::_01)
    }
    #[doc = "8bit-timeout"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TODTS_A::_10)
    }
    #[doc = "6bit-timeout"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TODTS_A::_11)
    }
}
#[doc = "Field `TOLCTL` reader - Timeout L Count Control"]
pub type TOLCTL_R = crate::BitReader<TOLCTL_A>;
#[doc = "Timeout L Count Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOLCTL_A {
    #[doc = "0: Count is disabled while the SCLn line is at a low level."]
    _0 = 0,
    #[doc = "1: Count is enabled while the SCLn line is at a low level."]
    _1 = 1,
}
impl From<TOLCTL_A> for bool {
    #[inline(always)]
    fn from(variant: TOLCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl TOLCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOLCTL_A {
        match self.bits {
            false => TOLCTL_A::_0,
            true => TOLCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOLCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOLCTL_A::_1
    }
}
#[doc = "Field `TOLCTL` writer - Timeout L Count Control"]
pub type TOLCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMOCTL_SPEC, TOLCTL_A, O>;
impl<'a, const O: u8> TOLCTL_W<'a, O> {
    #[doc = "Count is disabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOLCTL_A::_0)
    }
    #[doc = "Count is enabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOLCTL_A::_1)
    }
}
#[doc = "Field `TOHCTL` reader - Timeout H Count Control"]
pub type TOHCTL_R = crate::BitReader<TOHCTL_A>;
#[doc = "Timeout H Count Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOHCTL_A {
    #[doc = "0: Count is disabled while the SCLn line is at a high level."]
    _0 = 0,
    #[doc = "1: Count is enabled while the SCLn line is at a high level."]
    _1 = 1,
}
impl From<TOHCTL_A> for bool {
    #[inline(always)]
    fn from(variant: TOHCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl TOHCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOHCTL_A {
        match self.bits {
            false => TOHCTL_A::_0,
            true => TOHCTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOHCTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOHCTL_A::_1
    }
}
#[doc = "Field `TOHCTL` writer - Timeout H Count Control"]
pub type TOHCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMOCTL_SPEC, TOHCTL_A, O>;
impl<'a, const O: u8> TOHCTL_W<'a, O> {
    #[doc = "Count is disabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOHCTL_A::_0)
    }
    #[doc = "Count is enabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOHCTL_A::_1)
    }
}
#[doc = "Field `TOMDS` reader - Timeout Operation Mode Selection"]
pub type TOMDS_R = crate::FieldReader<u8, TOMDS_A>;
#[doc = "Timeout Operation Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOMDS_A {
    #[doc = "0: Timeout is detected during the following conditions: The bus is busy (BCST.BFREF = 0) in master mode.I3C’s own slave address is detected and the bus is busy in slave mode.The bus is free (BCST.BFREF = 1) while generation of a START condition is requested (CNDCTL.STCND = 1)."]
    _00 = 0,
    #[doc = "1: Timeout is detected while the bus is busy."]
    _01 = 1,
    #[doc = "2: Timeout is detected while the bus is free."]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<TOMDS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOMDS_A) -> Self {
        variant as _
    }
}
impl TOMDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOMDS_A {
        match self.bits {
            0 => TOMDS_A::_00,
            1 => TOMDS_A::_01,
            2 => TOMDS_A::_10,
            3 => TOMDS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOMDS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOMDS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOMDS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOMDS_A::_11
    }
}
#[doc = "Field `TOMDS` writer - Timeout Operation Mode Selection"]
pub type TOMDS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TMOCTL_SPEC, u8, TOMDS_A, 2, O>;
impl<'a, const O: u8> TOMDS_W<'a, O> {
    #[doc = "Timeout is detected during the following conditions: The bus is busy (BCST.BFREF = 0) in master mode.I3C’s own slave address is detected and the bus is busy in slave mode.The bus is free (BCST.BFREF = 1) while generation of a START condition is requested (CNDCTL.STCND = 1)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOMDS_A::_00)
    }
    #[doc = "Timeout is detected while the bus is busy."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOMDS_A::_01)
    }
    #[doc = "Timeout is detected while the bus is free."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOMDS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOMDS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timeout Detection Time Selection"]
    #[inline(always)]
    pub fn todts(&self) -> TODTS_R {
        TODTS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Timeout L Count Control"]
    #[inline(always)]
    pub fn tolctl(&self) -> TOLCTL_R {
        TOLCTL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timeout H Count Control"]
    #[inline(always)]
    pub fn tohctl(&self) -> TOHCTL_R {
        TOHCTL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Timeout Operation Mode Selection"]
    #[inline(always)]
    pub fn tomds(&self) -> TOMDS_R {
        TOMDS_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timeout Detection Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn todts(&mut self) -> TODTS_W<0> {
        TODTS_W::new(self)
    }
    #[doc = "Bit 4 - Timeout L Count Control"]
    #[inline(always)]
    #[must_use]
    pub fn tolctl(&mut self) -> TOLCTL_W<4> {
        TOLCTL_W::new(self)
    }
    #[doc = "Bit 5 - Timeout H Count Control"]
    #[inline(always)]
    #[must_use]
    pub fn tohctl(&mut self) -> TOHCTL_W<5> {
        TOHCTL_W::new(self)
    }
    #[doc = "Bits 6:7 - Timeout Operation Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tomds(&mut self) -> TOMDS_W<6> {
        TOMDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmoctl](index.html) module"]
pub struct TMOCTL_SPEC;
impl crate::RegisterSpec for TMOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmoctl::R](R) reader structure"]
impl crate::Readable for TMOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmoctl::W](W) writer structure"]
impl crate::Writable for TMOCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMOCTL to value 0x30"]
impl crate::Resettable for TMOCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
