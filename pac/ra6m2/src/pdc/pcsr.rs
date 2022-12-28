#[doc = "Register `PCSR` reader"]
pub struct R(crate::R<PCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSR` writer"]
pub struct W(crate::W<PCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSR_SPEC>;
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
impl From<crate::W<PCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBSY` reader - Frame Busy Flag"]
pub type FBSY_R = crate::BitReader<FBSY_A>;
#[doc = "Frame Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBSY_A {
    #[doc = "0: Operations for reception are stopped."]
    _0 = 0,
    #[doc = "1: Operations for reception are ongoing."]
    _1 = 1,
}
impl From<FBSY_A> for bool {
    #[inline(always)]
    fn from(variant: FBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl FBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBSY_A {
        match self.bits {
            false => FBSY_A::_0,
            true => FBSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FBSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FBSY_A::_1
    }
}
#[doc = "Field `FEMPF` reader - FIFO Empty Flag"]
pub type FEMPF_R = crate::BitReader<FEMPF_A>;
#[doc = "FIFO Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEMPF_A {
    #[doc = "0: FIFO is not empty."]
    _0 = 0,
    #[doc = "1: FIFO is empty."]
    _1 = 1,
}
impl From<FEMPF_A> for bool {
    #[inline(always)]
    fn from(variant: FEMPF_A) -> Self {
        variant as u8 != 0
    }
}
impl FEMPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEMPF_A {
        match self.bits {
            false => FEMPF_A::_0,
            true => FEMPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEMPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEMPF_A::_1
    }
}
#[doc = "Field `FEF` reader - Frame End Flag\n\nThe field is **modified** in some way after a read operation."]
pub type FEF_R = crate::BitReader<FEF_A>;
#[doc = "Frame End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    #[doc = "0: Frame end has not been generated."]
    _0 = 0,
    #[doc = "1: Frame end has been generated."]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEF_A::_1
    }
}
#[doc = "Field `FEF` writer - Frame End Flag"]
pub type FEF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, PCSR_SPEC, FEF_A, O>;
impl<'a, const O: u8> FEF_W<'a, O> {
    #[doc = "Frame end has not been generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Frame end has been generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
    }
}
#[doc = "Field `OVRF` reader - Overrun Flag\n\nThe field is **modified** in some way after a read operation."]
pub type OVRF_R = crate::BitReader<OVRF_A>;
#[doc = "Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRF_A {
    #[doc = "0: FIFO overrun has not been generated."]
    _0 = 0,
    #[doc = "1: FIFO overrun has been generated."]
    _1 = 1,
}
impl From<OVRF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRF_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRF_A {
        match self.bits {
            false => OVRF_A::_0,
            true => OVRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRF_A::_1
    }
}
#[doc = "Field `OVRF` writer - Overrun Flag"]
pub type OVRF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, PCSR_SPEC, OVRF_A, O>;
impl<'a, const O: u8> OVRF_W<'a, O> {
    #[doc = "FIFO overrun has not been generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRF_A::_0)
    }
    #[doc = "FIFO overrun has been generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRF_A::_1)
    }
}
#[doc = "Field `UDRF` reader - Underrun Flag\n\nThe field is **modified** in some way after a read operation."]
pub type UDRF_R = crate::BitReader<UDRF_A>;
#[doc = "Underrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRF_A {
    #[doc = "0: Underrun has not been generated."]
    _0 = 0,
    #[doc = "1: Underrun has been generated."]
    _1 = 1,
}
impl From<UDRF_A> for bool {
    #[inline(always)]
    fn from(variant: UDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl UDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDRF_A {
        match self.bits {
            false => UDRF_A::_0,
            true => UDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRF_A::_1
    }
}
#[doc = "Field `UDRF` writer - Underrun Flag"]
pub type UDRF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, PCSR_SPEC, UDRF_A, O>;
impl<'a, const O: u8> UDRF_W<'a, O> {
    #[doc = "Underrun has not been generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UDRF_A::_0)
    }
    #[doc = "Underrun has been generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UDRF_A::_1)
    }
}
#[doc = "Field `VERF` reader - Vertical Line Number Setting Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type VERF_R = crate::BitReader<VERF_A>;
#[doc = "Vertical Line Number Setting Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VERF_A {
    #[doc = "0: Vertical line number setting error has not been generated."]
    _0 = 0,
    #[doc = "1: Vertical line number setting error has been generated."]
    _1 = 1,
}
impl From<VERF_A> for bool {
    #[inline(always)]
    fn from(variant: VERF_A) -> Self {
        variant as u8 != 0
    }
}
impl VERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VERF_A {
        match self.bits {
            false => VERF_A::_0,
            true => VERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VERF_A::_1
    }
}
#[doc = "Field `VERF` writer - Vertical Line Number Setting Error Flag"]
pub type VERF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, PCSR_SPEC, VERF_A, O>;
impl<'a, const O: u8> VERF_W<'a, O> {
    #[doc = "Vertical line number setting error has not been generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VERF_A::_0)
    }
    #[doc = "Vertical line number setting error has been generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VERF_A::_1)
    }
}
#[doc = "Field `HERF` reader - Horizontal Byte Number Setting Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type HERF_R = crate::BitReader<HERF_A>;
#[doc = "Horizontal Byte Number Setting Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HERF_A {
    #[doc = "0: Horizontal byte number setting error has not been generated."]
    _0 = 0,
    #[doc = "1: Horizontal byte number setting error has been generated."]
    _1 = 1,
}
impl From<HERF_A> for bool {
    #[inline(always)]
    fn from(variant: HERF_A) -> Self {
        variant as u8 != 0
    }
}
impl HERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HERF_A {
        match self.bits {
            false => HERF_A::_0,
            true => HERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HERF_A::_1
    }
}
#[doc = "Field `HERF` writer - Horizontal Byte Number Setting Error Flag"]
pub type HERF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, PCSR_SPEC, HERF_A, O>;
impl<'a, const O: u8> HERF_W<'a, O> {
    #[doc = "Horizontal byte number setting error has not been generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HERF_A::_0)
    }
    #[doc = "Horizontal byte number setting error has been generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HERF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Frame Busy Flag"]
    #[inline(always)]
    pub fn fbsy(&self) -> FBSY_R {
        FBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Empty Flag"]
    #[inline(always)]
    pub fn fempf(&self) -> FEMPF_R {
        FEMPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame End Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Flag"]
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Underrun Flag"]
    #[inline(always)]
    pub fn udrf(&self) -> UDRF_R {
        UDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Vertical Line Number Setting Error Flag"]
    #[inline(always)]
    pub fn verf(&self) -> VERF_R {
        VERF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal Byte Number Setting Error Flag"]
    #[inline(always)]
    pub fn herf(&self) -> HERF_R {
        HERF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Frame End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<2> {
        FEF_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovrf(&mut self) -> OVRF_W<3> {
        OVRF_W::new(self)
    }
    #[doc = "Bit 4 - Underrun Flag"]
    #[inline(always)]
    #[must_use]
    pub fn udrf(&mut self) -> UDRF_W<4> {
        UDRF_W::new(self)
    }
    #[doc = "Bit 5 - Vertical Line Number Setting Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn verf(&mut self) -> VERF_W<5> {
        VERF_W::new(self)
    }
    #[doc = "Bit 6 - Horizontal Byte Number Setting Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn herf(&mut self) -> HERF_W<6> {
        HERF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr](index.html) module"]
pub struct PCSR_SPEC;
impl crate::RegisterSpec for PCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsr::R](R) reader structure"]
impl crate::Readable for PCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsr::W](W) writer structure"]
impl crate::Writable for PCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x7c;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCSR to value 0x02"]
impl crate::Resettable for PCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
