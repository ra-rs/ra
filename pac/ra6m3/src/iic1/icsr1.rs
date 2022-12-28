#[doc = "Register `ICSR1` reader"]
pub struct R(crate::R<ICSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR1` writer"]
pub struct W(crate::W<ICSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR1_SPEC>;
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
impl From<crate::W<ICSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAS0` reader - Slave Address 0 Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type AAS0_R = crate::BitReader<AAS0_A>;
#[doc = "Slave Address 0 Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAS0_A {
    #[doc = "0: Slave address 0 is not detected."]
    _0 = 0,
    #[doc = "1: Slave address 0 is detected."]
    _1 = 1,
}
impl From<AAS0_A> for bool {
    #[inline(always)]
    fn from(variant: AAS0_A) -> Self {
        variant as u8 != 0
    }
}
impl AAS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAS0_A {
        match self.bits {
            false => AAS0_A::_0,
            true => AAS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAS0_A::_1
    }
}
#[doc = "Field `AAS0` writer - Slave Address 0 Detection Flag"]
pub type AAS0_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ICSR1_SPEC, AAS0_A, O>;
impl<'a, const O: u8> AAS0_W<'a, O> {
    #[doc = "Slave address 0 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAS0_A::_0)
    }
    #[doc = "Slave address 0 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAS0_A::_1)
    }
}
#[doc = "Field `AAS1` reader - Slave Address 1 Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type AAS1_R = crate::BitReader<AAS1_A>;
#[doc = "Slave Address 1 Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAS1_A {
    #[doc = "0: Slave address 1 is not detected."]
    _0 = 0,
    #[doc = "1: Slave address 1 is detected."]
    _1 = 1,
}
impl From<AAS1_A> for bool {
    #[inline(always)]
    fn from(variant: AAS1_A) -> Self {
        variant as u8 != 0
    }
}
impl AAS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAS1_A {
        match self.bits {
            false => AAS1_A::_0,
            true => AAS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAS1_A::_1
    }
}
#[doc = "Field `AAS1` writer - Slave Address 1 Detection Flag"]
pub type AAS1_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ICSR1_SPEC, AAS1_A, O>;
impl<'a, const O: u8> AAS1_W<'a, O> {
    #[doc = "Slave address 1 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAS1_A::_0)
    }
    #[doc = "Slave address 1 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAS1_A::_1)
    }
}
#[doc = "Field `AAS2` reader - Slave Address 2 Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type AAS2_R = crate::BitReader<AAS2_A>;
#[doc = "Slave Address 2 Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAS2_A {
    #[doc = "0: Slave address 2 is not detected."]
    _0 = 0,
    #[doc = "1: Slave address 2 is detected"]
    _1 = 1,
}
impl From<AAS2_A> for bool {
    #[inline(always)]
    fn from(variant: AAS2_A) -> Self {
        variant as u8 != 0
    }
}
impl AAS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAS2_A {
        match self.bits {
            false => AAS2_A::_0,
            true => AAS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAS2_A::_1
    }
}
#[doc = "Field `AAS2` writer - Slave Address 2 Detection Flag"]
pub type AAS2_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ICSR1_SPEC, AAS2_A, O>;
impl<'a, const O: u8> AAS2_W<'a, O> {
    #[doc = "Slave address 2 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AAS2_A::_0)
    }
    #[doc = "Slave address 2 is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AAS2_A::_1)
    }
}
#[doc = "Field `GCA` reader - General Call Address Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type GCA_R = crate::BitReader<GCA_A>;
#[doc = "General Call Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCA_A {
    #[doc = "0: General call address is not detected."]
    _0 = 0,
    #[doc = "1: General call address is detected."]
    _1 = 1,
}
impl From<GCA_A> for bool {
    #[inline(always)]
    fn from(variant: GCA_A) -> Self {
        variant as u8 != 0
    }
}
impl GCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCA_A {
        match self.bits {
            false => GCA_A::_0,
            true => GCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCA_A::_1
    }
}
#[doc = "Field `GCA` writer - General Call Address Detection Flag"]
pub type GCA_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ICSR1_SPEC, GCA_A, O>;
impl<'a, const O: u8> GCA_W<'a, O> {
    #[doc = "General call address is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCA_A::_0)
    }
    #[doc = "General call address is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCA_A::_1)
    }
}
#[doc = "Field `DID` reader - Device-ID Address Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DID_R = crate::BitReader<DID_A>;
#[doc = "Device-ID Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DID_A {
    #[doc = "0: Device-ID command is not detected."]
    _0 = 0,
    #[doc = "1: Device-ID command is detected."]
    _1 = 1,
}
impl From<DID_A> for bool {
    #[inline(always)]
    fn from(variant: DID_A) -> Self {
        variant as u8 != 0
    }
}
impl DID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DID_A {
        match self.bits {
            false => DID_A::_0,
            true => DID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DID_A::_1
    }
}
#[doc = "Field `DID` writer - Device-ID Address Detection Flag"]
pub type DID_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ICSR1_SPEC, DID_A, O>;
impl<'a, const O: u8> DID_W<'a, O> {
    #[doc = "Device-ID command is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DID_A::_0)
    }
    #[doc = "Device-ID command is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DID_A::_1)
    }
}
#[doc = "Field `HOA` reader - Host Address Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type HOA_R = crate::BitReader<HOA_A>;
#[doc = "Host Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOA_A {
    #[doc = "0: Host address is not detected."]
    _0 = 0,
    #[doc = "1: Host address is detected."]
    _1 = 1,
}
impl From<HOA_A> for bool {
    #[inline(always)]
    fn from(variant: HOA_A) -> Self {
        variant as u8 != 0
    }
}
impl HOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOA_A {
        match self.bits {
            false => HOA_A::_0,
            true => HOA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOA_A::_1
    }
}
#[doc = "Field `HOA` writer - Host Address Detection Flag"]
pub type HOA_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, ICSR1_SPEC, HOA_A, O>;
impl<'a, const O: u8> HOA_W<'a, O> {
    #[doc = "Host address is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOA_A::_0)
    }
    #[doc = "Host address is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address 0 Detection Flag"]
    #[inline(always)]
    pub fn aas0(&self) -> AAS0_R {
        AAS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Address 1 Detection Flag"]
    #[inline(always)]
    pub fn aas1(&self) -> AAS1_R {
        AAS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Address 2 Detection Flag"]
    #[inline(always)]
    pub fn aas2(&self) -> AAS2_R {
        AAS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gca(&self) -> GCA_R {
        GCA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoa(&self) -> HOA_R {
        HOA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address 0 Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aas0(&mut self) -> AAS0_W<0> {
        AAS0_W::new(self)
    }
    #[doc = "Bit 1 - Slave Address 1 Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aas1(&mut self) -> AAS1_W<1> {
        AAS1_W::new(self)
    }
    #[doc = "Bit 2 - Slave Address 2 Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aas2(&mut self) -> AAS2_W<2> {
        AAS2_W::new(self)
    }
    #[doc = "Bit 3 - General Call Address Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn gca(&mut self) -> GCA_W<3> {
        GCA_W::new(self)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn did(&mut self) -> DID_W<5> {
        DID_W::new(self)
    }
    #[doc = "Bit 7 - Host Address Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hoa(&mut self) -> HOA_W<7> {
        HOA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr1](index.html) module"]
pub struct ICSR1_SPEC;
impl crate::RegisterSpec for ICSR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icsr1::R](R) reader structure"]
impl crate::Readable for ICSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr1::W](W) writer structure"]
impl crate::Writable for ICSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xaf;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSR1 to value 0"]
impl crate::Resettable for ICSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
