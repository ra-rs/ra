#[doc = "Register `SYRFL2R` reader"]
pub struct R(crate::R<SYRFL2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYRFL2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYRFL2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYRFL2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYRFL2R` writer"]
pub struct W(crate::W<SYRFL2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYRFL2R_SPEC>;
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
impl From<crate::W<SYRFL2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYRFL2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAN0` reader - Management Message Processing Setting"]
pub type MAN0_R = crate::BitReader<MAN0_A>;
#[doc = "Management Message Processing Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAN0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<MAN0_A> for bool {
    #[inline(always)]
    fn from(variant: MAN0_A) -> Self {
        variant as u8 != 0
    }
}
impl MAN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAN0_A {
        match self.bits {
            false => MAN0_A::_0,
            true => MAN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAN0_A::_1
    }
}
#[doc = "Field `MAN0` writer - Management Message Processing Setting"]
pub type MAN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL2R_SPEC, MAN0_A, O>;
impl<'a, const O: u8> MAN0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAN0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAN0_A::_1)
    }
}
#[doc = "Field `SIG0` reader - Signaling Message Processing Setting"]
pub type SIG0_R = crate::BitReader<SIG0_A>;
#[doc = "Signaling Message Processing Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIG0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<SIG0_A> for bool {
    #[inline(always)]
    fn from(variant: SIG0_A) -> Self {
        variant as u8 != 0
    }
}
impl SIG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIG0_A {
        match self.bits {
            false => SIG0_A::_0,
            true => SIG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIG0_A::_1
    }
}
#[doc = "Field `SIG0` writer - Signaling Message Processing Setting"]
pub type SIG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL2R_SPEC, SIG0_A, O>;
impl<'a, const O: u8> SIG0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIG0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIG0_A::_1)
    }
}
#[doc = "Field `ILL0` reader - Illegal Message Processing Setting"]
pub type ILL0_R = crate::BitReader<ILL0_A>;
#[doc = "Illegal Message Processing Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILL0_A {
    #[doc = "0: Messages are not transferred to the PTPEDMAC."]
    _0 = 0,
    #[doc = "1: Messages are transferred to the PTPEDMAC."]
    _1 = 1,
}
impl From<ILL0_A> for bool {
    #[inline(always)]
    fn from(variant: ILL0_A) -> Self {
        variant as u8 != 0
    }
}
impl ILL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILL0_A {
        match self.bits {
            false => ILL0_A::_0,
            true => ILL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILL0_A::_1
    }
}
#[doc = "Field `ILL0` writer - Illegal Message Processing Setting"]
pub type ILL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRFL2R_SPEC, ILL0_A, O>;
impl<'a, const O: u8> ILL0_W<'a, O> {
    #[doc = "Messages are not transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILL0_A::_0)
    }
    #[doc = "Messages are transferred to the PTPEDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILL0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Management Message Processing Setting"]
    #[inline(always)]
    pub fn man0(&self) -> MAN0_R {
        MAN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Signaling Message Processing Setting"]
    #[inline(always)]
    pub fn sig0(&self) -> SIG0_R {
        SIG0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 28 - Illegal Message Processing Setting"]
    #[inline(always)]
    pub fn ill0(&self) -> ILL0_R {
        ILL0_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management Message Processing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn man0(&mut self) -> MAN0_W<0> {
        MAN0_W::new(self)
    }
    #[doc = "Bit 4 - Signaling Message Processing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn sig0(&mut self) -> SIG0_W<4> {
        SIG0_W::new(self)
    }
    #[doc = "Bit 28 - Illegal Message Processing Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ill0(&mut self) -> ILL0_W<28> {
        ILL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Reception Filter Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syrfl2r](index.html) module"]
pub struct SYRFL2R_SPEC;
impl crate::RegisterSpec for SYRFL2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syrfl2r::R](R) reader structure"]
impl crate::Readable for SYRFL2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syrfl2r::W](W) writer structure"]
impl crate::Writable for SYRFL2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYRFL2R to value 0"]
impl crate::Resettable for SYRFL2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
